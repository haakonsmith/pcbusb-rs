use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("PCBUSB").display()
    );
    println!("cargo:rustc-link-lib-static=libPCBUSB");
    cc::Build::new().file("PCBUSB/PCBUSB.c").compile("PCBUSB");

    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/Users/haakonsmith/Development/easytune-j2534-driver/pcan_usb/PCBUSB/");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("PCBUSB/PCBUSB.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
