//! This build script is responsible for compiling the Protocol Buffers (protobuf) definitions  

/* 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/ner_proto.proto")?;
    Ok(())
}
*/



fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/ner_proto.proto");

    std::fs::create_dir_all("src/generated")?;

    tonic_build::configure()
        .out_dir("src/generated")
        .compile_protos(
            &["proto/ner_proto.proto"],
            &["proto"],
        )?;

    Ok(())
}