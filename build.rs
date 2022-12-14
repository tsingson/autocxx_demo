fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path]).build()?;
    b.flag_if_supported("-std=c++17")
            .compile("autocxx-pod-example");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cpp.h");
    Ok(())
}
