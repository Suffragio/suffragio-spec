use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let proto_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../proto");

    let protos = [
        "suffragio/v1/common.proto",
        "suffragio/v1/election_registry.proto",
        "suffragio/v1/registration_eligibility.proto",
        "suffragio/v1/blind_signature.proto",
        "suffragio/v1/vote_queue.proto",
        "suffragio/v1/tally.proto",
        "suffragio/v1/formula_catalog.proto",
        "suffragio/v1/discovery.proto",
    ]
    .iter()
    .map(|p| proto_dir.join(p))
    .collect::<Vec<_>>();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_well_known_types(true)
        .extern_path(".google.protobuf.Timestamp", "::prost_types::Timestamp")
        .out_dir(&out_dir)
        .compile_protos(&protos, &[proto_dir])?;

    Ok(())
}
