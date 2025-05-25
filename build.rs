// extern crate cbindgen;
extern crate cxx_build;

fn main() {
    // cbindgen::Builder::new()
    //     .with_language(cbindgen::Language::Cxx)
    //     .with_crate(".")
    //     .with_namespace("dwebble")
    //     .with_std_types(true)
    //     .generate()
    //     .expect("cbindgen error")
    //     .write_to_file("Bindgens/Bindings.h");

    // https://github.com/dtolnay/cxx/issues/880#issuecomment-2521375384

    // Don't link the default CRT
    // println!("cargo::rustc-link-arg=/nodefaultlib:msvcrt");
    // Link the debug CRT instead
    // println!("cargo::rustc-link-arg=/defaultlib:msvcrtd");

    // let out_lib_dir = "Bindings";
    // std::fs::remove_dir_all(out_lib_dir).unwrap();

    // cxx_build::bridge("src/ffi.rs")
    //     .out_dir(out_lib_dir)
    //     .std("c++20")
    //     .compile("dwebble");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
