extern crate cc;

use std::path::PathBuf;

fn main() {
    bindgen::Builder::default()
        .header("libexample/example.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from("./src").join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("libexample/example.c")
        .compile("libexample.a");
}
