fn main() {
    println!("cargo:rustc-link-arg=-N");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-no-pie");
    // Supprime la section d'en-tête de commentaires du compilateur (.comment)
    println!("cargo:rustc-link-arg=-Wl,--strip-all");

    // Élimine le code et les données jamais utilisés (Dead Code Elimination au niveau du linker)
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");

    // Supprime la table des notes ELF inutiles (build-id)
    println!("cargo:rustc-link-arg=-Wl,--build-id=none");
    // Le programme n'utilise pas le débogage d'exceptions ou de déroulage de pile.
    println!("cargo:rustc-link-arg=-Wl,--no-eh-frame-hdr");
    println!("cargo:rustc-link-arg=-Wl,--no-dynamic-linker");
}
