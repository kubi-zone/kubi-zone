use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};

use futures::StreamExt;
use k8s_openapi::serde_json::json;
use kube::{
    Api, Client, ResourceExt,
    api::{ListParams, Patch, PatchParams},
    runtime::{Controller, controller::Action, watcher},
};
use kubizone_common::{Class, DomainName, FullyQualifiedDomainName, Type};
use kubizone_crds::{
    PARENT_ZONE_LABEL,
    v1alpha1::{DomainExt as _, Record, Zone, ZoneEntry, ZoneSpec},
};

use tracing::log::*;

use crate::{set_fqdn, set_parent, with_parent_zone};

pub struct ZoneControllerContext {
    pub client: Client,
    pub requeue_time: Duration,
}

#[cfg(feature = "dev")]
const CONTROLLER_NAME: &str = "dev.kubi.zone/zone-resolver";
#[cfg(not(feature = "dev"))]
const CONTROLLER_NAME: &str = "kubi.zone/zone-resolver";

pub async fn controller(context: ZoneControllerContext) {
    let zones = Api::<Zone>::all(context.client.clone());

    let zone_controller = Controller::new(zones.clone(), watcher::Config::default())
        .watches(
            Api::<Zone>::all(context.client.clone()),
            watcher::Config::default(),
            with_parent_zone(),
        )
        .watches(
            Api::<Record>::all(context.client.clone()),
            watcher::Config::default(),
            with_parent_zone(),
        )
        .shutdown_on_signal()
        .run(reconcile_zones, zone_error_policy, Arc::new(context))
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled: {:?}", o),
                Err(e) => warn!("reconciliation failed: {}", e),
            }
        });

    zone_controller.await;
    warn!("zone controller exited");
}

#[tracing::instrument(name = "zone", skip_all)]
async fn reconcile_zones(
    zone: Arc<Zone>,
    ctx: Arc<ZoneControllerContext>,
) -> Result<Action, kube::Error> {
    match (zone.spec.zone_ref.as_ref(), &zone.spec.domain_name) {
        (Some(zone_ref), DomainName::Partial(partial_domain)) => {
            // Follow the zoneRef to the supposed parent zone, if it exists
            // or requeue later if it does not.
            let Some(parent_zone) = Api::<Zone>::namespaced(
                ctx.client.clone(),
                &zone_ref
                    .namespace
                    .as_ref()
                    .or(zone.namespace().as_ref())
                    .cloned()
                    .unwrap(),
            )
            .get_opt(&zone_ref.name)
            .await?
            else {
                warn!("zone {zone} references unknown zone {zone_ref}");
                return Ok(Action::requeue(Duration::from_secs(30)));
            };

            // If the parent does not have a fully qualified domain name defined
            // yet, we can't check if the delegations provided by it are valid.
            // Postpone the reconcilliation until a later time, when the fqdn
            // has (hopefully) been determined.
            let Some(parent_fqdn) = parent_zone.fqdn() else {
                info!(
                    "parent zone {} missing fqdn, requeuing.",
                    parent_zone.name_any()
                );
                return Ok(Action::requeue(Duration::from_secs(5)));
            };

            // This is only "alleged", since we don't know yet if the referenced
            // zone's delegations allow the adoption.
            let alleged_fqdn: FullyQualifiedDomainName = partial_domain.with_origin(parent_fqdn);

            trace!("zone alleged fqdn: {partial_domain} + {parent_fqdn} = {alleged_fqdn}");

            if parent_zone.spec.delegations.iter().any(|delegation| {
                delegation.covers_namespace(zone.namespace().as_deref().unwrap())
                    && delegation.validate_zone(parent_fqdn, &alleged_fqdn)
            }) {
                set_fqdn(CONTROLLER_NAME, ctx.client.clone(), &zone, &alleged_fqdn).await?;
                set_parent(
                    CONTROLLER_NAME,
                    ctx.client.clone(),
                    &zone,
                    Some(parent_zone.zone_ref()),
                )
                .await?;
            } else {
                warn!(
                    "parent zone {parent_zone} was found, but its delegations do not allow adoption of {zone} with {alleged_fqdn}"
                );
                return Ok(Action::requeue(ctx.requeue_time));
            }
        }
        (None, DomainName::Full(fqdn)) => {
            set_fqdn(CONTROLLER_NAME, ctx.client.clone(), &zone, fqdn).await?;

            // Fetch all zones from across the cluster and then filter down results to only parent
            // zones which are valid parent zones for this one.
            //
            // This means filtering out parent zones without fqdns, as well as ones which do not
            // have appropriate delegations for our `zone`'s namespace and suffix.
            if let Some(longest_parent_zone) = Api::<Zone>::all(ctx.client.clone())
                .list(&ListParams::default())
                .await?
                .into_iter()
                .filter(|parent| {
                    parent
                        .fqdn()
                        .is_some_and(|parent_fqdn| fqdn.is_subdomain_of(parent_fqdn))
                })
                .max_by_key(|parent| parent.fqdn().unwrap().as_ref().len())
            {
                if longest_parent_zone.validate_zone(&zone) {
                    set_parent(
                        CONTROLLER_NAME,
                        ctx.client.clone(),
                        &zone,
                        Some(longest_parent_zone.zone_ref()),
                    )
                    .await?;
                } else {
                    warn!(
                        "{longest_parent_zone} is the most immediate parent zone of {zone}, but the zone's delegation rules do not allow the adoption of it."
                    );
                }
            } else {
                info!(
                    "zone {} ({}) does not fit into any found parent zone. If this is a top level zone, then this is expected.",
                    zone.name_any(),
                    &zone.spec.domain_name
                );
            };
        }
        (Some(zone_ref), DomainName::Full(fqdn)) => {
            warn!(
                "zone {zone} has both a fully qualified domain_name ({fqdn}) and a zoneRef({zone_ref}). It cannot have both."
            );
            return Ok(Action::requeue(ctx.requeue_time));
        }
        (None, DomainName::Partial(_)) => {
            warn!(
                "{zone} has neither zoneRef nor a fully qualified domainName, making it impossible to deduce its parent zone."
            );
            return Ok(Action::requeue(ctx.requeue_time));
        }
    }

    update_zone_status(zone, ctx.client.clone()).await?;
    Ok(Action::requeue(ctx.requeue_time))
}

async fn update_zone_status(zone: Arc<Zone>, client: Client) -> Result<(), kube::Error> {
    let Some(origin) = zone.fqdn() else {
        return Ok(());
    };

    // Reference to this zone, which other zones and records will use to refer to it by.
    let zone_ref = ListParams::default().labels(&format!(
        "{PARENT_ZONE_LABEL}={}",
        zone.zone_ref().as_label()
    ));

    let mut entries = Vec::new();

    // Insert all child records into the entries list
    for record in Api::<Record>::all(client.clone())
        .list(&zone_ref)
        .await?
        .into_iter()
    {
        if !zone.validate_record(&record) {
            warn!(
                "record {record} has {zone} configured as its parent, but the zone does not allow this delegation, action could be malicious."
            );
            continue;
        }

        entries.push(ZoneEntry {
            fqdn: record.fqdn().unwrap().clone(), // Unwrap safe since fqdn presence is checked in validate_record
            type_: record.spec.type_,
            class: record.spec.class,
            ttl: record.spec.ttl.unwrap_or(zone.spec.ttl),
            rdata: record.spec.rdata,
        })
    }

    let mut hasher = DefaultHasher::new();
    (&zone.spec, &entries).hash(&mut hasher);
    let new_hash = hasher.finish().to_string();

    let current_hash = zone.status.as_ref().and_then(|status| status.hash.as_ref());

    let last_serial = zone
        .status
        .as_ref()
        .and_then(|status| status.serial)
        .unwrap_or_default();

    // If the hash changed, we need to update the serial.
    let serial = if current_hash != Some(&new_hash) {
        info!(
            "zone {zone}'s hash changed (before: {current_hash:?}, now: {new_hash}), updating serial."
        );
        // Compute a serial based on the current datetime in UTC as per:
        // https://datatracker.ietf.org/doc/html/rfc1912#section-2.2
        let now = time::OffsetDateTime::now_utc();
        #[rustfmt::skip]
        let now_serial
            = now.year()  as u32 * 1000000
            + now.month() as u32 * 10000
            + now.day()   as u32 * 100;

        // If it's a new day, use YYYYMMDD00, otherwise just use the increment
        // of the old serial.
        std::cmp::max(now_serial, last_serial + 1)
    } else {
        last_serial
    };

    // Insert a SOA record at the beginning of the entry list.
    let ZoneSpec {
        ttl,
        refresh,
        retry,
        expire,
        negative_response_cache,
        ..
    } = zone.spec;

    entries.insert(0, ZoneEntry {
        fqdn: origin.clone(),
        type_: Type::SOA,
        class: Class::IN,
        ttl,
        rdata: format!("ns.{origin} noc.{origin} ({serial} {refresh} {retry} {expire} {negative_response_cache})"),
    });

    Api::<Zone>::namespaced(client, zone.namespace().as_ref().unwrap())
        .patch_status(
            &zone.name_any(),
            &PatchParams::apply(CONTROLLER_NAME),
            &Patch::Merge(json!({
                "status": {
                    "hash": new_hash,
                    "entries": entries,
                    "serial": Some(serial)
                },
            })),
        )
        .await?;

    Ok(())
}

fn zone_error_policy(
    zone: Arc<Zone>,
    error: &kube::Error,
    _ctx: Arc<ZoneControllerContext>,
) -> Action {
    error!(
        "zone {} reconciliation encountered error: {error}",
        zone.name_any()
    );
    Action::requeue(Duration::from_secs(60))
}
