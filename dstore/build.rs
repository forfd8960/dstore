fn main() {
    prost_build::Config::new()
        .bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir("src/pb")
        .compile_protos(&["dstore.proto"], &["."])
        .unwrap();
}
