use anyhow::Result;
use ethers::prelude::Abigen;
use std::fs;
use std::path::Path;

/// Generate a rust bindings given abis
#[allow(dead_code)]
pub fn generate_bindings() -> Result<()> {
    let abi_dir = Path::new("src/abi");
    let bindings_dir = Path::new("src/bindings");

    let entries = fs::read_dir(abi_dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().unwrap() == "json" {
            let contract_name = path.file_stem().unwrap().to_str().unwrap().to_lowercase();

            Abigen::new(contract_name.clone(), path.to_str().unwrap())
                .unwrap()
                .generate()
                .unwrap()
                .write_to_file(bindings_dir.join(format!("{}.rs", contract_name)))
                .unwrap();

            log::info!("Generated bindings for {}", contract_name);
        }
    }

    Ok(())
}
