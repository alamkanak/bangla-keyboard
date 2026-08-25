use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("BANGLA_KEYBOARD_ENGINE_H")
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("bangla_keyboard_engine.h");
}
