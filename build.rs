use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let _dst = Config::new("libpd").build_target("libpd_static").build();

    println!("cargo:rustc-link-search=native=libpd/libs");
    println!("cargo:rustc-link-lib=static=pd");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ilibpd/libpd_wrapper")
        .clang_arg("-Ilibpd/pure-data/src")
        .header("wrapper.h")
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
