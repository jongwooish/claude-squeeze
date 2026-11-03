# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.1-Sentinel/v0.1_squeeze-sentinel/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.1-Sentinel/v0.1_squeeze-sentinel/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#instalación) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | **Español** | [Français](README.fr.md)

> 💬 **Se agradecen comentarios.** Mejoras, el lanzamiento de la v1.0, actualizaciones, errores de instalación o cualquier otro problema: [abre un issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 idiomas).

**Gasta tus tokens de Claude en pensar, no en logs interminables.**

squeeze evita que la salida larga de los comandos y la lectura completa de archivos grandes entren en el contexto de Claude. Claude sigue viendo cada error y advertencia, además de las primeras y últimas líneas. El log completo se guarda en tu disco por si hace falta.

- [Efecto](#efecto) · [Versiones](#versiones) · [Dónde funciona](#dónde-funciona) · [v0.1 Sentinel](#v01-sentinel) · [Instalación](#instalación) · [Uso](#uso) · [Cómo funciona](#cómo-funciona) · [Hoja de ruta](#hoja-de-ruta) · [FAQ](#faq)

## Por qué

Claude Code reenvía toda la conversación en cada petición, y la salida de los comandos forma parte de ella. Un `npm install`, una ejecución de tests o un `journalctl` pueden añadir miles de tokens que viajan en cada turno posterior, aunque Claude solo necesitaba "pasó" o tres líneas de error.

squeeze detiene esa salida **antes** de que entre en el contexto.

## Efecto

Siete logs reales procesados con squeeze ([método](../v0.1-Sentinel/v0.1_squeeze-sentinel/docs/bench.md)):

| Log | Líneas antes → después | Tokens antes → después (est.) | Ahorro | Errores/advertencias distintos conservados |
|---|---:|---:|---:|---:|
| `pytest`, 2 fallos | 217 → 39 | 4,040 → 516 | **87.2%** | 11 / 11 |
| `go test -v`, 1 fallo | 488 → 16 | 4,247 → 202 | **95.2%** | 4 / 4 |
| `make` con advertencias de gcc + 1 error | 370 → 44 | 7,046 → 850 | **87.9%** | 7 / 7 |
| `tsc` con 2 errores de tipo | 124 → 9 | 1,310 → 172 | **86.9%** | 3 / 3 |
| Lista de paquetes `dpkg -l` | 1049 → 63 | 35,237 → 2,106 | **94.0%** | 8 / 8 |
| Journal de servidor, OOM kill (sintético) | 1400 → 37 | 31,161 → 997 | **96.8%** | 5 / 5 |
| Salida corta de `npm` | 16 → 16 | 287 → 287 | 0.0% (sin cambios) | 12 / 12 |
| **Total** | | **83,325 → 5,128** | **93.8%** | **50 / 50** |

**Por qué es útil**

- **En estas pruebas no se perdió nada importante.** Cada mensaje de error y advertencia distinto se conservó. Los repetidos se muestran una vez con un contador (`… [same message 29 more times] …`).
- **La salida corta nunca se toca.** Con menos de 80 líneas y 8 KB recibes el original.
- **El ahorro se repite en cada turno.** Un log de 31,000 tokens que permanece en el contexto durante los siguientes 20 turnos son unos 620,000 tokens de entrada. Reducido a unos 1,000 tokens, esos mismos 20 turnos cuestan unos 20,000.
- **Sesiones más largas antes de los límites.** Con menos ruido el contexto se llena más despacio, así que la compactación y los límites del plan llegan más tarde.
- **Claude se concentra mejor.** La respuesta a "¿por qué falló el build?" está delante, no enterrada en 1,000 líneas.

**Límites, con honestidad:** los tokens son estimaciones (bytes ÷ 4) y estos números salen de reproducir logs guardados, no de una prueba A/B en vivo con Claude Code (está planeada). Ejecuta `squeeze report` en tu proyecto para ver tus números reales.

## Versiones

Cada versión tiene su propia carpeta y README. Las versiones publicadas son etiquetas sobre todo el repositorio.

| Versión | Nombre | Qué añade | Estado |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1_squeeze-sentinel/README/README.es.md) | **Sentinel** | Hooks de Claude Code que atrapan comandos ruidosos y lecturas completas de archivos, `squeeze run`, `compress`, `clip`, `report` | ✅ v0.1.0 |
| v0.2 | **Interceptor** | Extensión de navegador que ofrece reducir el texto largo al pegarlo en claude.ai | 🛠 Planeada |
| v0.3 | **Conduit** | Servidor MCP para la app de escritorio de Claude: Claude lee tus logs ya reducidos | 🛠 Planeada |

### Cómo encajan las versiones

Hay dos cosas que puedes instalar: el **programa `squeeze`** (CLI) y, desde la v0.2, una **extensión de navegador**.

| Versión | Qué instalas | ¿Necesita la v0.1? |
|---|---|---|
| v0.1 Sentinel | El programa `squeeze` | — |
| v0.2 Interceptor | Una extensión de navegador | **No.** Lleva su propia copia del compresor y funciona sola. Instala también la v0.1 si usas Claude Code. |
| v0.3 Conduit | Nada nuevo: es un comando dentro del programa `squeeze` (`squeeze mcp`) | **Sí.** Actualiza `squeeze` a la v0.3 o posterior; no hay descarga aparte. |

- **El programa es acumulativo.** Un `squeeze` más nuevo incluye todas las funciones anteriores. Nunca instalas la v0.1 y la v0.3 a la vez.
- **Las carpetas describen lo que añadió cada versión.** No se instala una carpeta.
- **Tu configuración se conserva.** `.squeeze/config.json` y los hooks de `squeeze init` siguen funcionando tras actualizar. Si una versión los cambia, su entrada en el CHANGELOG lo dice y `squeeze init` los actualiza.

| Usas… | Instala |
|---|---|
| Solo Claude Code | `squeeze` (ahora la v0.1) |
| Solo claude.ai | `squeeze clip` ahora, la extensión v0.2 cuando salga |
| La app de escritorio de Claude y quieres que Claude lea los logs por sí mismo | `squeeze` v0.3 cuando salga |
| Todo lo anterior | `squeeze` (última) + la extensión v0.2 |

## Dónde funciona

| Dónde | Efecto | Cómo |
|---|---|---|
| **Claude Code** | Automático | `squeeze init` instala los hooks. Los comandos ruidosos y las lecturas de archivos grandes se redirigen. |
| **claude.ai (web, escritorio, móvil)** | Manual, al pegar texto largo | Copia un log, ejecuta `squeeze clip`, pega. Los mensajes normales no cambian; squeeze no puede ver tus chats. |
| **Cualquier otra herramienta de terminal** | Manual | `<comando> 2>&1 \| squeeze compress` |

En el plan Pro, Claude Code y claude.ai comparten un mismo límite de uso, así que los tokens que ahorras en Claude Code quedan también para tus chats.

## v0.1 Sentinel

Sentinel hace guardia entre Claude Code y tu shell. Un solo binario, Rust, **cero dependencias**, macOS / Linux / Windows.

- **Guardia de Bash**: los comandos ruidosos (`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log` y más) se redirigen a `squeeze run -- <comando>`. Los que ya limitan su salida (`| tail -20`, `-n 50`, `> archivo`, `--quiet`) no se tocan.
- **Guardia de Read**: leer un archivo de más de 400 líneas sin `offset`/`limit` se redirige. Claude recibe un índice con números de línea y elige un rango.
- **Compresión inteligente**: primeras 8 y últimas 20 líneas, cada error y advertencia con 2 líneas de contexto, trazas de pila; repeticiones agrupadas con un contador; los errores tienen prioridad sobre las advertencias.
- **Código de salida conservado**: `squeeze run` termina con el código del propio comando.
- **Logs completos en disco**: `.squeeze/raw/`, se guardan los 50 más recientes, legibles solo por ti. `.squeeze/` se añade a `.gitignore`.
- **`squeeze clip`** para claude.ai y **`squeeze report`** para ver tu ahorro real.
- **Seguro por diseño**: los hooks nunca aprueban nada por ti, se apartan ante cualquier error (fail open) y `squeeze uninstall` elimina todo lo que añadió. Sin código de red.

Lo que Claude ve tras `squeeze run -- pytest -v` (salida real, 217 líneas → 39):

```
============================= test session starts ==============================
platform linux -- Python 3.13.18, pytest-9.1.1, pluggy-1.6.0 -- /usr/local/bin/python3
cachedir: .pytest_cache
rootdir: ~/project/
plugins: platformdirs-4.13.3, anyio-4.15.1
collecting ... collected 183 items

test_inventory.py::test_restock[0] PASSED                                [  0%]
… [178 lines omitted] …
test_inventory.py::test_restock[179] PASSED                              [ 98%]
test_inventory.py::test_negative PASSED                                  [ 98%]
test_inventory.py::test_bulk_order FAILED                                [ 99%]
test_inventory.py::test_missing_sku FAILED                               [100%]

=================================== FAILURES ===================================
… (las dos trazas se conservan completas) …
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

Documentación completa de la v0.1: [v0.1-Sentinel/v0.1_squeeze-sentinel/README.es.md](../v0.1-Sentinel/v0.1_squeeze-sentinel/README/README.es.md).

## Instalación

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# Desde el código fuente (cualquier sistema con Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

Los instaladores verifican el `checksums.txt` de la versión (SHA-256). `SQUEEZE_VERSION=v0.1.0` fija una versión; `SQUEEZE_INSTALL_DIR` cambia la carpeta.

## Inicio rápido

```bash
cd ~/my-project
squeeze init      # hooks, configuración, entrada en .gitignore, nota en CLAUDE.md
claude            # usa Claude Code como siempre
squeeze report    # mira lo que se quedó fuera del contexto
```

`squeeze init` cambia cuatro archivos y `squeeze uninstall` lo deshace:

| Archivo | Cambio |
|---|---|
| `.claude/settings.json` | Añade dos hooks `PreToolUse` (`Bash`, `Read`). Se conservan tus otros ajustes y hooks; antes se escribe una copia de seguridad. |
| `.squeeze/config.json` | Configuración por defecto. |
| `.gitignore` | Añade `.squeeze/` (los logs pueden contener secretos). |
| `CLAUDE.md` | Añade una nota de 4 líneas entre marcadores `<!-- squeeze:start -->`. |

## Uso

| Comando | Qué hace |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Instalar en un proyecto |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Quitar hooks y nota; `--purge` también borra `.squeeze/` |
| `squeeze run -- <comando>` | Ejecutar un comando; resumen corto si la salida es larga |
| `<comando> 2>&1 \| squeeze compress` | Comprimir salida por tubería |
| `squeeze clip [--print]` | Comprimir el portapapeles (para claude.ai) |
| `squeeze report [--since 7d] [--global] [--json]` | Ver el ahorro |
| `squeeze bench <dir> [--markdown]` | Reproducir archivos `*.log` y medir |
| `squeeze version` | Mostrar la versión |

Antepón `SQUEEZE_SKIP=1` a un comando para obtener su salida original una vez. La configuración está en `.squeeze/config.json` ([todas las claves](../v0.1-Sentinel/v0.1_squeeze-sentinel/README/README.es.md#configuración)).

## Cómo funciona

```
Claude quiere ejecutar:  npm test
        │
        ▼
Hook PreToolUse ── squeeze hook pre-bash
        │  ¿ruidoso? ── no ──▶ silencio; flujo de permisos normal
        │  sí
        ▼
denegar + pista: "ejecútalo como  squeeze run -- 'npm test'"
        │
        ▼
Claude ejecuta: squeeze run -- 'npm test'   (tus reglas de permisos se aplican igual)
        │
        ├─▶ salida completa ── .squeeze/raw/<hora>.log
        └─▶ salida corta ───── Claude (errores, advertencias, inicio, final, ruta del log)
```

¿Por qué "denegar con una pista" en lugar de reescribir el comando en silencio? Un hook que reescribe un comando también tiene que aprobarlo, y eso se saltaría tus confirmaciones de permisos. squeeze nunca aprueba nada por ti.

### En qué se diferencia de los wrappers

Herramientas como [chaude](https://github.com/int-ttt/chaude) envuelven la CLI de Claude Code en su propia pantalla y ahorran tokens rotando sesiones y pasando trabajo a modelos más baratos. squeeze trabaja en el otro lado del problema:

- **No es un wrapper.** Sigues usando `claude` tal cual, mediante los hooks propios de Claude Code.
- **Frena el desperdicio en la puerta** en lugar de limpiar un contexto lleno.
- **Ambos enfoques pueden usarse juntos.**

## Hoja de ruta

- [x] **v0.1 Sentinel**: hooks de Claude Code, `run`, `compress`, `clip`, `report`, `bench`
- [ ] **v0.2 Interceptor**: extensión de navegador para pegar en claude.ai, el mismo motor compilado a WebAssembly
- [ ] **v0.3 Conduit**: `squeeze mcp`, servidor MCP de solo lectura para la app de escritorio de Claude
- [ ] Benchmark A/B en vivo con Claude Code (mismas tareas, squeeze activado y desactivado)
- [ ] v1.0: comandos y configuración estables

## FAQ

**P. ¿Puede Claude perderse algo importante?**
R. En todos los logs de prueba se conservaron todos los mensajes de error y advertencia distintos. Si el resumen no basta, la última línea da la ruta del log completo y Claude puede leerlo por rangos.

**P. ¿squeeze envía mis datos a algún sitio?**
R. No. No tiene código de red. Los logs se quedan en `.squeeze/` en tu máquina.

**P. ¿Puede romper mi sesión de Claude Code?**
R. Los hooks se apartan ante fallos (fail open): una entrada extraña, un archivo inexistente o un error interno no producen salida y Claude Code sigue con normalidad.

**P. ¿Ahorra tokens en los chats normales de claude.ai?**
R. Solo cuando pegas texto largo tras ejecutar `squeeze clip`. La v0.2 lo hará automático en el navegador.

**P. ¿Por qué la versión 0.x?**
R. Mientras la versión mayor sea 0, los nombres de comandos y la configuración pueden cambiar. La v1.0 significará que son estables.

## Estructura del repositorio

```
claude-squeeze/
├── README/                      ← resumen del proyecto en 6 idiomas (README.md = inglés)
├── CHANGELOG/                   ← CHANGELOG en 6 idiomas
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   resumen de la versión
│   └── v0.1_squeeze-sentinel/   ← v0.1: CLI + hooks de Claude Code (Rust, sin dependencias)
│       ├── README.md               documentación completa de la v0.1 (inglés)
│       ├── README/                 traducciones (5 idiomas)
│       ├── src/
│       ├── docs/                   método del benchmark, guía de usuario (coreano)
│       └── testdata/fixtures/      logs usados por los tests y el benchmark
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock (workspace de Rust)
├── .github/                     ← formularios de issues, CI, release, README de portada
├── CLAUDE.md
└── LICENSE
```

## Contribuir

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures
```

Se agradecen nuevos logs de prueba: añade un `*.log` real (sin secretos ni rutas personales) a `v0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures/`. Los tests fallan si se pierde cualquier error o advertencia distinto. Errores e ideas: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).

## Comunidad

**¿Encontraste un problema o tienes una idea? Abre un [issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose).** Mejoras, el lanzamiento de la v1.0, actualizaciones de versión, errores del instalador o cualquier otro problema del software: cada aporte ayuda a dar forma a la próxima versión.


| Quiero… | Ir a |
|---|---|
| Informar de un error o de una sesión rota | [Issues → Bug report](https://github.com/jongwooish/claude-squeeze/issues/new?template=bug_report.yml) |
| Informar de una línea perdida, poco ahorro o un comando (no) detectado | [Issues → Compression problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=compression.yml) |
| Informar de un problema de instalación, actualización o versión | [Issues → Install, update or version problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=version_problem.yml) |
| Proponer una función | [Issues → Feature idea](https://github.com/jongwooish/claude-squeeze/issues/new?template=feature_request.yml) |
| Hacer una pregunta o pedir ayuda | [Discussions → Q&A](https://github.com/jongwooish/claude-squeeze/discussions/categories/q-a) |
| Hablar de la hoja de ruta y de lo que debe incluir la v1.0 | [Discussions → Ideas](https://github.com/jongwooish/claude-squeeze/discussions/categories/ideas) |
| Enterarte de las nuevas versiones | [Discussions → Announcements](https://github.com/jongwooish/claude-squeeze/discussions/categories/announcements) y [Releases](https://github.com/jongwooish/claude-squeeze/releases) (pulsa **Watch → Custom → Releases** para recibir avisos) |

Puedes escribir en cualquiera de los seis idiomas. Problemas de seguridad: [SECURITY.md](../.github/SECURITY.md). Cómo contribuir: [CONTRIBUTING.md](../.github/CONTRIBUTING.md).

## Licencia

MIT. Consulta [LICENSE](../LICENSE). squeeze es una herramienta comunitaria no oficial y no está afiliada a Anthropic.
