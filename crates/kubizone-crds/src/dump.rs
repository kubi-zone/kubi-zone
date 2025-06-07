use kube::{CustomResourceExt, Resource};
use std::path::PathBuf;

fn main() {
    write_to_path::<kubizone_crds::v1alpha1::Record>().unwrap();
    write_to_path::<kubizone_crds::v1alpha1::Zone>().unwrap();
}

fn serialize_crd<C>() -> Result<String, serde_yaml::Error>
where
    C: Resource<DynamicType = ()> + CustomResourceExt,
{
    Ok(format!("---\n{}", serde_yaml::to_string(&C::crd())?))
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
