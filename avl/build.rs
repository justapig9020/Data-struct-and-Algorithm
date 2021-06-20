use cc;
use bindgen::Builder;
use std::path::Path;

fn main() {
    let bindgen = Builder::default()
        .header("include/avl.h")
        .generate()
        .unwrap();
    bindgen
        .write_to_file("src/avl.rs")
        .unwrap();
    cc::Build::new()
        .file("src/avl.c")
        .includes(Path::new("include"))
        .compile("avl");
}
