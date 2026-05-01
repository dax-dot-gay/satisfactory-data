use std::error::Error;

use satisfactory_data::Registry;

fn main() -> Result<(), Box<dyn Error>> {
    let registry = Registry::from_url(
        "https://github.com/dax-dot-gay/satisfactory-data/raw/refs/heads/main/data/satisfactory-1.1-en-US.zip",
    )?;
    println!("Metadata: {:?}", registry.get_metadata());
    println!("Source: {:?}", registry.get_registry_reference());

    let mutable_registry = registry.persist("./persisted")?;
    println!(
        "New Source: {:?}",
        mutable_registry.get_registry_reference()
    );
    println!(
        "Example recipe: {:?}",
        mutable_registry.get_item("recipe/lookout_tower")?
    );

    Ok(())
}
