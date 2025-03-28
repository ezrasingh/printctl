fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/client")
        .compile_protos(&["proto/v0/printctl.proto"], &["proto"])?;

    tonic_build::configure()
        .build_client(false)
        .out_dir("src/server")
        .compile_protos(&["proto/v0/printctl.proto"], &["proto"])?;
    Ok(())
}
