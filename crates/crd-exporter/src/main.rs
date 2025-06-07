use kube::{CustomResourceExt, Resource};
use regex::Regex;
use std::path::PathBuf;

fn main() {
    write_to_path::<kubizone_crds::v1alpha1::Record>().unwrap();
    write_to_path::<kubizone_crds::v1alpha1::Zone>().unwrap();
    write_to_path::<zonefile_crds::v1alpha1::ZoneFile>().unwrap();
}

fn serialize_crd<C>() -> Result<String, serde_yaml::Error>
where
    C: Resource<DynamicType = ()> + CustomResourceExt,
{
    // Removes nice docs.rs [`Type`](crate::path) references in the Custom Resource Definition spec,
    // since the references are meaningless to the end user.
    let regex = Regex::new(r#"\[`(?<label>\w+)`\](\([^\)]+\))?"#).unwrap();

    let document = serde_yaml::to_string(&C::crd())?;
    let document = regex.replace_all(&document, "$label");

    Ok(format!("---\n{}", document))
}

fn write_to_path<C>() -> Result<(), std::io::Error>
where
    C: Resource<DynamicType = ()> + CustomResourceExt,
{
    let directory = PathBuf::from("crds").join(C::api_version(&()).as_ref());

    std::fs::create_dir_all(&directory)?;

    std::fs::write(
        directory.join(format!("{name}.yaml", name = C::kind(&()))),
        serialize_crd::<C>().unwrap(),
    )
    .unwrap();

    Ok(())
}
