fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["protos/message1/message1.proto"], &["protos"])?;
    Ok(())
}
