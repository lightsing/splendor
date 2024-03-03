fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/supervisor.proto");
    println!("cargo:rerun-if-changed=proto/controller.proto");
    tonic_build::compile_protos("proto/supervisor.proto")?;
    tonic_build::compile_protos("proto/controller.proto")?;
    Ok(())
}
