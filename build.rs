fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/proto") // Output directory for generated code
        .compile(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
