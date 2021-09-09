use cc;
use bindgen::Builder;
use std::path::Path;

fn main() {
   let bindgen = Builder::default()
       .header("include/bst.h")
       .generate()
       .unwrap();

    bindgen
        .write_to_file("src/bst.rs")
        .unwrap();
    cc::Build::new()
        .file("src/bst.c")
        .includes(Path::new("include"))
        .compile("bst");
}
