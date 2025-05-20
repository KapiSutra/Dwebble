// extern crate cbindgen;
extern crate cxx_build;

use std::fs;


fn main() {
    // cbindgen::Builder::new()
    //     .with_language(cbindgen::Language::Cxx)
    //     .with_crate(".")
    //     .with_namespace("Dwebble")
    //     .generate()
    //     .expect("cbindgen error")
    //     .write_to_file("Source/Dwebble/Bindings.h");

    let out_lib_dir = "Bindings";
    fs::remove_dir_all(out_lib_dir).unwrap();

    cxx_build::bridge("src/lib.rs")
        .out_dir(out_lib_dir)
        .compile("dwebble");
}
