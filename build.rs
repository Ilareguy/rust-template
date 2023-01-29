fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-change=proto/");
    tonic_build::configure()
        .build_client(false)
        .compile(
            &["proto/tiger/api.proto"],
            &["proto"]
        ).unwrap();
    Ok(())
}
