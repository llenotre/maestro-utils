fn main() {
    // TODO add feature to link this lib only if necessary
    println!("cargo:rustc-link-lib=crypt");

    println!("cargo:rerun-if-changed=src/hash.c");
    println!("cargo:rerun-if-changed=src/termios.c");

    cc::Build::new()
        .static_flag(true)
        .flag("-lcrypt")
        .file("src/hash.c")
        .compile("utils")
}
