extern crate bindgen;

use std::{env, path::PathBuf};

use bindgen::CargoCallbacks;

fn main() {
    // initialize path for external C library
    let current_dir = env::current_dir().expect("Cannot find current dir");

    let libdir_path = current_dir
        .join("geographiclib")
        .canonicalize()
        .expect("cannot canonicalize path");

    // generating build parts paths
    let headers_path = libdir_path.join("geodesic.h");
    let sources_path = libdir_path.join("geodesic.c");
    let obj_path = libdir_path.join("geodesic.o");
    let lib_path = libdir_path.join("libgeodesic.a");

    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // adding instructions for cargo to build
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=geodesic");
    println!("cargo:rerun-if-changed={}", &headers_path_str);

    // launching clang to generate the .o file for the C lib.
    if !std::process::Command::new("clang")
        .arg("-fPIC")
        .arg("-c")
        .arg(sources_path)
        .arg("-o")
        .arg(&obj_path)
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        panic!("could not compile object file");
    }

    // transforming the .o file into a .a file
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        panic!("could not emit library file");
    }

    // generating C -> Rust bindings
    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
