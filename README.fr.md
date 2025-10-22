# Advent of Code – Espace de travail Rust (Jours 1 à 7)

## Présentation
- Ce dépôt contient un espace de travail Rust composé de sept crates binaires indépendantes : `day1` à `day7`.
- Chaque crate résout une énigme quotidienne d'Advent of Code en s'appuyant uniquement sur la bibliothèque standard de Rust (aucune dépendance externe).
- Les données d'entrée de chaque énigme sont intégrées à la compilation via `include_str!` depuis `src/input.txt` propre à chaque crate.

## Prérequis
- Rust (canal stable) et Cargo installés. Si nécessaire, installez-les via <https://rustup.rs>.

## Organisation de l'espace de travail
```
Cargo.toml        # manifeste racine de l'espace de travail
Cargo.lock
README.md
README.fr.md

dayN/
├── Cargo.toml    # manifeste de la crate N
└── src/
    ├── main.rs   # point d'entrée de la solution pour le jour N
    └── input.txt # données d'entrée intégrées
```

## Compilation
- Compiler toutes les crates de l'espace de travail (mode debug) :
  ```bash
  cargo build --workspace
  ```
- Compiler toutes les crates en mode release (exécution plus rapide) :
  ```bash
  cargo build --workspace --release
  ```

## Exécution
- Depuis la racine de l'espace de travail, exécuter un jour précis (remplacez `N` par `1`–`7`) :
  ```bash
  cargo run -p dayN
  # Exemple : cargo run -p day3
  ```
- Exécuter avec optimisations :
  ```bash
  cargo run -p dayN --release
  ```
- Alternative depuis le répertoire d'une crate :
  ```bash
  cd dayN
  cargo run
  ```

## Exécuter tous les jours
- Bash/Zsh :
  ```bash
  for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
  ```
- PowerShell :
  ```powershell
  1..7 | ForEach-Object { cargo run -q -p "day$_" --release }
  ```

## Entrées
- Chaque crate lit son entrée depuis `src/input.txt` à la compilation grâce à `include_str!`.
- Pour essayer une autre entrée, modifiez le fichier `src/input.txt` correspondant puis relancez la commande souhaitée (la recompilation est automatique lorsque le fichier change).

## Notes de performance
- Certaines solutions affichent des mesures de temps en utilisant `std::time::Instant`.
- Utilisez `--release` pour obtenir des temps d'exécution représentatifs.

## Contribution et développement
- Aucune crate externe n'est utilisée : les solutions s'appuient sur les structures de données et algorithmes de la bibliothèque standard.
- Maintenez chaque jour autonome dans sa propre crate afin de préserver l'organisation de l'espace de travail.

## Licence
- Aucun fichier de licence n'est fourni. Si vous envisagez de publier ou de réutiliser le code, pensez à ajouter un fichier `LICENSE` approprié.
