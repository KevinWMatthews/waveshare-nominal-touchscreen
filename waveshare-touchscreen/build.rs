fn main() {
    // Fetch and "propagate" (export) kconfig and linker flags from the ESP-IDF.
    //
    // If linker flags are not exported, then a later call to `ldproxy` fails with:
    // `Cannot locate argument '--ldproxy-linker <linker>'`
    // One of the exported linker flags is `--ldproxy-linker`
    // (specifically, `cargo:rustc-link-arg=--ldproxy-linker`).
    // `ldproxy` requires this argument and fails if it is missing.
    //
    // See also:
    // https://github.com/rust-lang/cargo/issues/9554
    embuild::build::CfgArgs::output_propagated("ESP_IDF")
        .expect("Failed to propagate ESP-IDF CfgArgs");
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
        .expect("Failed to export ESP-IDF LinkArgs");
}
