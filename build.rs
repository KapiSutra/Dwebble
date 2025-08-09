fn main() {
    // let cbindgen_out_dir = "cbindgen";
    // std::fs::remove_dir_all(cbindgen_out_dir)
    //     .and_then(|_| std::fs::create_dir(cbindgen_out_dir))
    //     .unwrap_or_default();
    //
    // cbindgen::Builder::new()
    //     .with_language(cbindgen::Language::Cxx)
    //     .with_namespaces(&["dwebble_rs"])
    //     .with_crate(".")
    //     .with_cpp_compat(true)
    //     .with_std_types(true)
    //     .with_pragma_once(true)
    //     .generate()
    //     .expect("cbindgen error")
    //     .write_to_file(std::path::PathBuf::from(cbindgen_out_dir).join("dwebble_rs.h"));

    let cxx_lib_dir = "cxx";
    std::fs::remove_dir_all(cxx_lib_dir)
        .and_then(|_| std::fs::create_dir(cxx_lib_dir))
        .unwrap_or_default();

    cxx_build::bridge("ffi/index.rs")
        // .out_dir(cxx_lib_dir)
        .includes(["src"])
        // .file("target/x86_64-pc-windows-msvc/cxxbridge/dwebble/ffi/index.rs.cc")
        .std("c++20")
        .compile("dwebble_cxx");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=ffi");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-env-changed=PROFILE=release");

    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=bcrypt");
    println!("cargo:rustc-link-lib=ntdll");
    println!("cargo:rustc-link-lib=userenv");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=msvcrt");
}
