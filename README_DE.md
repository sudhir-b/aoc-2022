Advent of Code – Rust Workspace (Tage 1–7)

Überblick
- Dieses Repository ist ein Rust Workspace, der sieben unabhängige Binär-Crates enthält: day1 bis day7.
- Jeder Crate löst eine tägliche Aufgabe, ausschließlich unter Verwendung der Rust-Standardbibliothek (keine externen Abhängigkeiten).
- Die Eingabedaten der Aufgaben werden zur Kompilierzeit über include_str! aus src/input.txt in jedem Crate eingebettet.

Voraussetzungen
- Rust (stable) und Cargo installiert. Installation über https://rustup.rs

Workspace-Struktur
- Cargo.toml (Workspace-Root)
- dayN/
  - Cargo.toml (Crate-Manifest)
  - src/
    - main.rs (Einstiegspunkt der Lösung)
    - input.txt (eingebettete Eingabedaten)

Kompilierung
- Alle Crates im Workspace kompilieren (Debug-Modus):
  - cargo build --workspace
- Alle Crates im Release-Modus kompilieren (schnellere Ausführung):
  - cargo build --workspace --release

Ausführung
- Aus dem Workspace-Root einen bestimmten Tag ausführen (ersetzen Sie N durch 1–7):
  - cargo run -p dayN
  - Beispiel: cargo run -p day3
- Mit Optimierungen ausführen:
  - cargo run -p dayN --release
- Alternativ aus dem Crate-Verzeichnis:
  - cd dayN && cargo run

Alle Tage ausführen
- Bash/Zsh:
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell:
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Eingabedaten
- Jeder Crate liest seine Eingabe zur Kompilierzeit aus src/input.txt mit include_str!.
- Um andere Eingabedaten zu testen, bearbeiten Sie die entsprechende src/input.txt und führen Sie das Programm erneut aus (Neukompilierung erfolgt bei Änderungen automatisch).

Performance-Hinweise
- Einige Lösungen zeigen einfache Zeitmessungsinformationen mit std::time::Instant.
- Verwenden Sie --release für repräsentative Laufzeiten.

Mitwirkung / Entwicklung
- Es werden keine externen Crates verwendet; die Lösungen basieren auf Datenstrukturen und Algorithmen der Standardbibliothek.
- Behalten Sie jeden Tag in seinem eigenen Crate, um die Workspace-Struktur zu erhalten.

Lizenz
- Es ist keine Lizenzdatei enthalten. Wenn Sie den Code veröffentlichen oder wiederverwenden möchten, fügen Sie bitte eine passende LICENSE-Datei hinzu.