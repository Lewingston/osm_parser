
fn main() {

    protobuf_codegen::Codegen::new()
        //.protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .pure()
        .include("osmpbf")
        .input("osmpbf/fileformat.proto")
        .input("osmpbf/osmformat.proto")
        .cargo_out_dir("osmpbf")
        .run_from_script();
}
