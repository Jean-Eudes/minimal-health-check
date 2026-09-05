# Minimal health check en Rust

Ce projet est un petit serveur HTTP écrit en Rust. Il écoute sur le port `8080` et répond simplement `OK` à chaque requête.

Je me suis inspiré de cet article : [Un serveur HTTP de moins de 20 Ko](https://lafor.ge/http-smol/#optimisations). J’ai repris l’idée pour créer ma propre version en Rust, avec pour objectif de voir si on peut faire plus petit, en gardant un code un peu lisible (en gros pas d'assembleur direct).

## Pourquoi le binaire est petit ?

Le projet évite plusieurs éléments habituellement présents dans un programme Rust classique :

- `no_std` évite de charger toute la bibliothèque standard Rust ;
- `no_main` permet d’utiliser directement un point d’entrée personnalisé ;
- `rustix` le coeur de l'implementation, permet de gerer certaines features de la libc, comme la manipulation de file descriptor, ou l'ouverture de socket, sans la libC ;
- la compilation release optimise la taille, supprime les symboles inutiles et utilise le LTO ;
- `panic = "immediate_abort"` évite d’embarquer le mécanisme de récupération des erreurs (vient de la nightly) ;
- l’image Docker utilise `scratch`, donc elle ne contient pas de système de fichiers ou de runtime superflux.

Le programme n’utilise pas directement la `libc`. Grâce à `no_std` et à `rustix`, il demande directement au système d’exploitation les ressources dont il a besoin, notamment pour créer et utiliser la socket TCP. Le binaire est donc autonome et peut être placé dans une image Docker `scratch`, sans ajouter une image Linux complète ni les bibliothèques habituelles.

## Compilation

La compilation minimale utilise le nightly et reconstruit `core` et
`panic_abort` pour éviter d'embarquer le runtime standard :

```shell
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Zunstable-options -Cpanic=immediate-abort" \\
  cargo build --release -Z build-std=core,panic_abort \\
  --target x86_64-unknown-linux-gnu
```

Le binaire est ensuite disponible dans
`target/x86_64-unknown-linux-gnu/release/health-http`.

Pour produire l'artefact ELF minimal, appliquer `sstrip` séparément :

```shell
sstrip target/x86_64-unknown-linux-gnu/release/health-http
stat -c '%s octets' target/x86_64-unknown-linux-gnu/release/health-http
```

Cette variante produit un ELF à un seul segment `PT_LOAD`. `sstrip` retire
ensuite `.comment`, la table des sections et les autres métadonnées inutiles
au chargeur Linux. La suppression de `PT_GNU_STACK` rend la pile exécutable :
c'est un compromis volontaire en faveur de la taille.

