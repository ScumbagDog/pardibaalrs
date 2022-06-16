extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=stdc++");
    let bindings = bindgen::Builder::default()
        .header("PARDIBAAL/src/pardibaal/DBM.h")
        .clang_arg("-IPARDIBAAL/src/pardibaal")
        .clang_arg("-x").clang_arg("c++")
        .opaque_type("std::.*")
        .allowlist_function(".*DBM_future") //this was apparently all that was needed to properly include all the associated methods. Don't ask me how it works, I don't know.
        .derive_copy(true)
	.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("{:?}", out_path.join("bindings.rs"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");


    let src = [
        "PARDIBAAL/src/pardibaal/DBM.cpp",
    ];
    let includes = ["PARDIBAAL/src/pardibaal",
                    "PARDIBAAL/include",
                    ];
    let mut builder = cc::Build::new();
    let build = builder
        .cpp(true)
        .files(src.iter())
        .includes(includes.iter());

    build.compile("PARDIBAAL");
}
