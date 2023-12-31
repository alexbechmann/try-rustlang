fn main() {
    // https://docs.rs/protobuf-codegen/latest/protobuf_codegen/
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["../../specs/protobuf"])
        // Inputs must reside in some of include paths.
        .input("../../specs/protobuf/purchase.proto")
        .input("../../specs/protobuf/page-view.proto")
        .input("../../specs/protobuf/customer-event.proto")
        // .input("src/protos/banana.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        // .out_dir("src/protos")
        .run_from_script();

    println!("Codegen done!")
}
