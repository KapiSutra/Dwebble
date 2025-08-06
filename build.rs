fn main() {
    cbindgen::Builder::new()
        .with_language(cbindgen::Language::Cxx)
        .with_namespaces(&["dwebble_rs"])
        .with_crate(".")
        .with_cpp_compat(true)
        .with_std_types(true)
        .with_pragma_once(true)
        .generate()
        .expect("cbindgen error")
        .write_to_file("Bindgens/dwebble_rs.h");

    // https://github.com/dtolnay/cxx/issues/880#issuecomment-2521375384

    // if std::env::var("TARGET").is_ok_and(|s| return s.contains("windows-msvc")) {
    //     // MSVC compiler suite
    //
    //     if Ok("debug".to_owned()) == std::env::var("PROFILE") {
    //         // debug runtime flag is set
    //
    //         // Don't link the default CRT
    //         println!("cargo::rustc-link-arg=/nodefaultlib:msvcrt");
    //         // Link the debug CRT instead
    //         println!("cargo::rustc-link-arg=/defaultlib:msvcrtd");
    //     }
    // }
    //
    // let out_lib_dir = "Bindings";
    // std::fs::remove_dir_all(out_lib_dir).unwrap();
    //
    // cxx_build::bridge("src/ffi.rs")
    //     .out_dir(out_lib_dir)
    //     .std("c++20")
    //     .compile("dwebble");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
}
