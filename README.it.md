Advent of Code – Workspace Rust (Giorni 1–7)

Panoramica
- Questo repository è un workspace Rust che contiene sette crate binarie indipendenti: day1 a day7.
- Ogni crate risolve un puzzle quotidiano utilizzando solo la libreria standard di Rust (nessuna dipendenza esterna).
- I dati di input dei puzzle sono incorporati durante la compilazione tramite include_str! da src/input.txt in ogni crate.

Prerequisiti
- Rust (stable) e Cargo installati. Installateli tramite https://rustup.rs

Organizzazione del Workspace
- Cargo.toml (root del workspace)
- dayN/
  - Cargo.toml (manifest della crate)
  - src/
    - main.rs (punto di ingresso della soluzione)
    - input.txt (dati di input incorporati)

Compilazione
- Compilare tutte le crate del workspace (modalità debug):
  - cargo build --workspace
- Compilare tutte le crate in modalità release (esecuzione più veloce):
  - cargo build --workspace --release

Esecuzione
- Dalla root del workspace, eseguire un giorno specifico (sostituire N con 1–7):
  - cargo run -p dayN
  - Esempio: cargo run -p day3
- Eseguire con ottimizzazioni:
  - cargo run -p dayN --release
- In alternativa, dalla directory di una crate:
  - cd dayN && cargo run

Eseguire tutti i giorni
- Bash/Zsh:
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell:
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Input
- Ogni crate legge il suo input da src/input.txt durante la compilazione con include_str!.
- Per provare un input diverso, modificate il relativo src/input.txt e rieseguite (la ricompilazione è automatica se il file è modificato).

Note sulle Prestazioni
- Alcune soluzioni mostrano semplici informazioni di timing utilizzando std::time::Instant.
- Utilizzate --release per tempi rappresentativi.

Contribuzione / Sviluppo
- Nessuna crate esterna è utilizzata; le soluzioni si affidano alle strutture dati e agli algoritmi della libreria standard.
- Mantenete ogni giorno autonomo nella propria crate per preservare l'organizzazione del workspace.

Licenza
- Nessun file di licenza è fornito. Se intendete pubblicare o riutilizzare il codice, considerate l'aggiunta di un file LICENSE appropriato.
