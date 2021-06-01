fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helix_user_v1.proto")?;
    Ok(())
}
