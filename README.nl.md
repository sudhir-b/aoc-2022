Advent of Code – Rust Workspace (Dagen 1–7)

Overzicht
- Deze repository is een Rust workspace die zeven onafhankelijke binaire crates bevat: day1 tot day7.
- Elke crate lost een dagelijkse puzzel op met enkel de standaardbibliotheek van Rust (geen externe dependencies).
- De invoergegevens van de puzzels worden tijdens compilatie ingebed via include_str! vanuit src/input.txt in elke crate.

Vereisten
- Rust (stable) en Cargo geïnstalleerd. Installeer deze via https://rustup.rs

Workspace Organisatie
- Cargo.toml (root van de workspace)
- dayN/
  - Cargo.toml (crate manifest)
  - src/
    - main.rs (oplossing entry point)
    - input.txt (ingebedde invoergegevens)

Compileren
- Compileer alle crates in de workspace (debug modus):
  - cargo build --workspace
- Compileer alle crates in release modus (snellere uitvoering):
  - cargo build --workspace --release

Uitvoeren
- Voer een specifieke dag uit vanuit de root van de workspace (vervang N door 1–7):
  - cargo run -p dayN
  - Voorbeeld: cargo run -p day3
- Uitvoeren met optimalisaties:
  - cargo run -p dayN --release
- Alternatief, vanuit de directory van een crate:
  - cd dayN && cargo run

Alle dagen uitvoeren
- Bash/Zsh:
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell:
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Invoer
- Elke crate leest zijn invoer vanaf src/input.txt tijdens compilatie met include_str!.
- Om andere invoer te proberen, wijzig de betreffende src/input.txt en voer opnieuw uit (hercompilatie gebeurt automatisch bij wijziging).

Prestatie Opmerkingen
- Sommige oplossingen tonen eenvoudige timing informatie met behulp van std::time::Instant.
- Gebruik --release voor representatieve tijdmetingen.

Bijdrage / Ontwikkeling
- Er worden geen externe crates gebruikt; de oplossingen vertrouwen op datastructuren en algoritmen uit de standaardbibliotheek.
- Houd elke dag autonoom in zijn eigen crate om de workspace organisatie te behouden.

Licentie
- Er is geen licentiebestand voorzien. Als je van plan bent de code te publiceren of hergebruiken, overweeg dan om een passend LICENSE bestand toe te voegen.
