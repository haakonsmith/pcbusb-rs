fn main() {
    // Read from the environment rather than #[cfg], which would check the host
    // instead of the target when cross compiling.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        // The MacCAN installer puts libPCBUSB here, with an @rpath install name.
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=PCBUSB");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/lib");
    }
}
