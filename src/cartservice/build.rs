/*
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("frontend-rust_descriptor.bin"))
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile(&["protos/demo.proto"], &["protos"])
        .unwrap();

    /*tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"))
                .join("demo_descriptor.bin"),
        )
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile(&["protos/demo.proto"], &["protos"])?;
    */
    Ok(())
}
*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/Cart.proto")?;
    Ok(())
}
