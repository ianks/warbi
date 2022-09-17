use std::{
    error::Error,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/bindings/ruby-value-resource.wit");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();

    build_host_export_bindings(&out_dir, &manifest_dir)?;
    build_host_import_bindings(&out_dir, &manifest_dir)?;

    Ok(())
}

fn build_host_export_bindings(
    out_dir: &PathBuf,
    manifest_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    Command::new("wit-bindgen")
        .args(&[
            "host",
            "wasmtime-rust",
            "--export",
            manifest_dir
                .join("src/bindings/ruby-value-resource.wit")
                .to_str()
                .unwrap(),
            "--rustfmt",
            "--out-dir",
            out_dir.to_str().unwrap(),
            "--custom-error",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    let mut contents = fs::read_to_string(out_dir.join("bindings.rs"))?;

    contents = contents.replace(
        "wit_bindgen_host_wasmtime_rust::Table",
        "crate::table::Table",
    );

    fs::write(manifest_dir.join("src/bindings/export.rs"), contents)?;

    Ok(())
}

fn build_host_import_bindings(
    out_dir: &PathBuf,
    manifest_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    Command::new("wit-bindgen")
        .args(&[
            "guest",
            "rust",
            "--import",
            manifest_dir
                .join("src/bindings/ruby-value-resource.wit")
                .to_str()
                .unwrap(),
            "--rustfmt",
            "--out-dir",
            out_dir.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    let contents = fs::read_to_string(out_dir.join("bindings.rs"))?;

    fs::write(manifest_dir.join("src/bindings/import.rs"), contents)?;

    Ok(())
}
