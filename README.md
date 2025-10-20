Advent of Code – Espace de travail Rust (Jours 1–7)

Aperçu
- Ce dépôt est un espace de travail Rust qui contient sept crates binaires indépendantes : day1 à day7.
- Chaque crate résout une énigme quotidienne en utilisant uniquement la bibliothèque standard de Rust (aucune dépendance externe).
- Les données d’entrée des énigmes sont intégrées à la compilation via include_str! à partir de src/input.txt dans chaque crate.

Prérequis
- Rust (stable) et Cargo installés. Installez-les via https://rustup.rs

Organisation de l’espace de travail
- Cargo.toml (racine de l’espace de travail)
- dayN/
  - Cargo.toml (manifest de la crate)
  - src/
    - main.rs (point d’entrée de la solution)
    - input.txt (données d’entrée intégrées)

Compilation
- Compiler toutes les crates de l’espace de travail (mode debug) :
  - cargo build --workspace
- Compiler toutes les crates en mode release (exécution plus rapide) :
  - cargo build --workspace --release

Exécution
- Depuis la racine de l’espace de travail, exécuter un jour précis (remplacez N par 1–7) :
  - cargo run -p dayN
  - Exemple : cargo run -p day3
- Exécuter avec optimisations :
  - cargo run -p dayN --release
- Alternativement, depuis le répertoire d’une crate :
  - cd dayN && cargo run

Exécuter tous les jours
- Bash/Zsh :
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell :
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Entrées
- Chaque crate lit son entrée depuis src/input.txt à la compilation avec include_str!.
- Pour essayer une autre entrée, modifiez le src/input.txt correspondant puis relancez (la recompilation est automatique en cas de modification).

Notes de performance
- Certaines solutions affichent des informations de chronométrage simples en utilisant std::time::Instant.
- Utilisez --release pour des temps représentatifs.

Contribution / Développement
- Aucune crate externe n’est utilisée ; les solutions s’appuient sur les structures de données et algorithmes de la bibliothèque standard.
- Conservez chaque jour autonome dans sa propre crate afin de préserver l’organisation de l’espace de travail.

Licence
- Aucun fichier de licence n’est fourni. Si vous envisagez de publier ou de réutiliser le code, pensez à ajouter un fichier LICENSE approprié.
