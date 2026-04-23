fn main() {
    #[rustfmt::skip] // there has to be a better way
    let r = cc::Build::new()
    .cpp(false)
    .warnings(true)
    .files([
        "entry.S",

    ])
    .flags([
        "-ffreestanding",
        "-Wall", "-Wextra"
        ])
    .try_compile("generated");

    if let Err(e) = r {
        panic!("The building of the x86_64 assembly code. Error {}", e)
    }
    println!("cargo::rustc-link-arg=-Tld-oes.ld");
}
