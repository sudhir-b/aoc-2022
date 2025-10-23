Advent of Code – Espacio de trabajo Rust (Días 1–7)

Resumen
- Este repositorio es un espacio de trabajo Rust que contiene siete crates binarias independientes: day1 a day7.
- Cada crate resuelve un acertijo diario utilizando únicamente la biblioteca estándar de Rust (sin dependencias externas).
- Los datos de entrada de los acertijos se integran en la compilación mediante include_str! desde src/input.txt en cada crate.

Requisitos previos
- Rust (estable) y Cargo instalados. Instálalos vía https://rustup.rs

Organización del espacio de trabajo
- Cargo.toml (raíz del espacio de trabajo)
- dayN/
  - Cargo.toml (manifiesto de la crate)
  - src/
    - main.rs (punto de entrada de la solución)
    - input.txt (datos de entrada integrados)

Compilación
- Compilar todas las crates del espacio de trabajo (modo debug):
  - cargo build --workspace
- Compilar todas las crates en modo release (ejecución más rápida):
  - cargo build --workspace --release

Ejecución
- Desde la raíz del espacio de trabajo, ejecutar un día específico (reemplaza N por 1–7):
  - cargo run -p dayN
  - Ejemplo: cargo run -p day3
- Ejecutar con optimizaciones:
  - cargo run -p dayN --release
- Alternativamente, desde el directorio de una crate:
  - cd dayN && cargo run

Ejecutar todos los días
- Bash/Zsh:
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell:
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Entradas
- Cada crate lee su entrada desde src/input.txt en tiempo de compilación con include_str!.
- Para probar otra entrada, modifica el src/input.txt correspondiente y vuelve a ejecutar (la recompilación es automática en caso de modificación).

Notas de rendimiento
- Algunas soluciones muestran información de cronometraje simple utilizando std::time::Instant.
- Utiliza --release para tiempos representativos.

Contribución / Desarrollo
- No se utilizan crates externas; las soluciones se basan en las estructuras de datos y algoritmos de la biblioteca estándar.
- Mantén cada día autónomo en su propia crate para preservar la organización del espacio de trabajo.

Licencia
- No se proporciona ningún archivo de licencia. Si planeas publicar o reutilizar el código, considera agregar un archivo LICENSE apropiado.
