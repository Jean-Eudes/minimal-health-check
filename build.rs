fn main() {
    println!("cargo:rustc-link-arg=-N");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-no-pie");
    println!("cargo:rustc-link-arg=-Wl,--strip-all");

    // Élimine le code et les données jamais utilisés.
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    println!("cargo:rustc-link-arg=-Wl,--build-id=none");
    println!(
        "cargo:rustc-link-arg=-T{}/linker.ld",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-arg=-Wl,--no-dynamic-linker");

    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=noseparate-code");
    println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=0x1000");
    println!("cargo:rustc-link-arg=-Wl,-z,common-page-size=0x1000");
}
