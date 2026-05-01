fn main() {
    println!("cargo::rustc-link-arg=-Toes-common.ld");
    #[rustfmt::skip]
    let bindings = bindgen::builder().header("multiboot.h")
    .rust_target(bindgen::RustTarget::nightly())
    .use_core()
    .sort_semantically(true)
    .generate().expect("Failed to generate multiboot.h 's bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("multiboot-sys.rs"))
        .expect("Failed to write multiboot bindings")
}
