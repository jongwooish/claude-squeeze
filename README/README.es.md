# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#instalación) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | **Español** | [Français](README.fr.md)

> 💬 **Se agradecen comentarios.** Mejoras, el lanzamiento de la v1.0, actualizaciones, errores de instalación o cualquier otro problema: [abre un issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 idiomas).

**Gasta tus tokens de Claude en pensar, no en logs interminables.**

squeeze evita que la salida larga de los comandos, las lecturas de archivos grandes o repetidas, las sesiones demasiado grandes y los servidores MCP sin uso entren en el contexto de Claude. Claude sigue viendo cada error y advertencia. **squeeze-Auditor**, la app instalable, muestra en qué se gastaron tus tokens y te deja elegir cuánto ahorra squeeze.

- [Efecto](#efecto) · [Versiones](#versiones) · [Dónde funciona](#dónde-funciona) · [v0.2 squeeze-Auditor](#v02-squeeze-auditor) · [v0.1 Sentinel](#v01-sentinel) · [Instalación](#instalación) · [Uso](#uso) · [Cómo funciona](#cómo-funciona) · [Hoja de ruta](#hoja-de-ruta) · [FAQ](#faq)

## Por qué

Claude Code reenvía toda la conversación en cada petición, y la salida de los comandos forma parte de ella. Un `npm install`, una ejecución de tests o un `journalctl` pueden añadir miles de tokens que viajan en cada turno posterior, aunque Claude solo necesitaba "pasó" o tres líneas de error.

squeeze detiene esa salida **antes** de que entre en el contexto.

## Efecto

Siete logs reales procesados con squeeze ([método](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/docs/bench.md)):

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

**v0.2 añade cuatro bloqueos** que impiden que los mismos tokens se envíen una y otra vez: releer un archivo sin cambios, una sesión que superó tu presupuesto, una sesión larga que debería empezar de nuevo desde una nota de traspaso breve, y costes fijos como servidores MCP sin uso. Su efecto depende de cómo trabajes, así que el [README de v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.es.md#efecto) muestra la aritmética y el panel de squeeze-Auditor muestra tus propios números.

## Versiones

Cada versión tiene su propia carpeta y README. Las versiones publicadas son etiquetas sobre todo el repositorio.

| Versión | Nombre | Qué añade | Estado |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.es.md) | **Sentinel** | Hooks de Claude Code que atrapan comandos ruidosos y lecturas completas de archivos, `squeeze run`, `compress`, `clip`, `report` | ✅ v0.1.0 |
| [v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.es.md) | **squeeze-Auditor** | Cuatro bloqueos de sesión (lectura repetida, pausa por presupuesto, traspaso, coste fijo), la app instalable squeeze-Auditor con panel de uso y modos de ahorro, soporte para la app de escritorio de Claude | ✅ v0.2.0 |
| v0.3 | *(nombre por decidir)* | claude.ai en el navegador y las apps móviles, con un enfoque nuevo para las quejas sobre tokens | 🛠 Planeada |

### Cómo encajan las versiones

| Versión | Qué instalas | ¿Necesita la versión anterior? |
|---|---|---|
| v0.1 Sentinel | El programa `squeeze` | — |
| v0.2 squeeze-Auditor | El mismo programa `squeeze`, ahora con instalador, una app en el menú Inicio (`squeeze-auditor`) y hooks nuevos | **No, incluye v0.1.** Instálala encima de v0.1 o sola. |
| v0.3 | Planeada | Se anunciará |

- **El programa es acumulativo.** Un `squeeze` más nuevo incluye todas las funciones anteriores. Nunca instalas v0.1 y v0.2 a la vez.
- **Las carpetas describen lo que añadió cada versión.** No se instala una carpeta.
- **Tu configuración se conserva.** `.squeeze/config.json` y los hooks de `squeeze init` de v0.1 siguen funcionando tras actualizar. v0.2 añade un `~/.squeeze/config.json` global y se conecta a Claude Code una sola vez para todos los proyectos.

| Usas… | Instala |
|---|---|
| Claude Code | squeeze-Auditor (v0.2) y luego **Conectar Claude Code** |
| La app de escritorio de Claude | squeeze-Auditor (v0.2) y luego **Conectar app de escritorio** |
| claude.ai en el navegador o las apps móviles | `squeeze clip` para pegar textos largos ahora; v0.3 cuando salga |

## Dónde funciona

| Dónde | Efecto | Cómo |
|---|---|---|
| **Claude Code** | Automático, todos los proyectos | squeeze-Auditor (o `squeeze setup`) añade hooks a `~/.claude/settings.json`. Funcionan los cuatro bloqueos de v0.2 y las dos guardas de v0.1. |
| **App de escritorio de Claude** (Windows, macOS) | Cuando Claude lee archivos o guarda un traspaso | Al conectar se añaden las herramientas de `squeeze mcp`: Claude lee tus registros ya comprimidos, solo de las carpetas que permitas, y puede guardar y cargar notas de traspaso. squeeze no puede ver ni cambiar las conversaciones. |
| **claude.ai (web, móvil)** | Manual, al pegar texto largo | Copia un log, ejecuta `squeeze clip` y pega. Previsto para v0.3. |
| **Cualquier otra herramienta de terminal** | Manual | `<comando> 2>&1 \| squeeze compress` |

En el plan Pro, Claude Code y claude.ai comparten un mismo límite de uso, así que los tokens que ahorras en Claude Code quedan también para tus chats.

## v0.2 squeeze-Auditor

v0.1 miraba una salida cada vez. **v0.2 mira la sesión completa**: lo que Claude ya leyó, cuánto ha crecido la conversación y lo que acompaña a cada mensaje. Bloquea cuatro tipos de desperdicio:

| Bloqueo | Qué detecta | Cómo bloquea |
|---|---|---|
| **Lectura repetida** | Releer entero un archivo que no ha cambiado | Rechaza el `Read`: «ya lo tienes». Se permite de nuevo cuando el archivo cambia o tras compactar. |
| **Pausa por presupuesto** | Enviar más mensajes a una sesión enorme (150K tokens por defecto) | Retiene tu mensaje una vez y sugiere `/compact` o `/clear`. Envíalo otra vez y pasa. |
| **Traspaso** | Una sesión larga que relee todo su historial (120K por defecto) | Claude escribe una nota breve; tras `/clear`, la nueva sesión empieza desde la nota y no desde toda la conversación. |
| **Coste fijo** | Archivos `CLAUDE.md`, skills, agentes y listas de herramientas MCP enviados con cada mensaje | Mide cada uno; apaga con un clic los servidores MCP sin uso (el interruptor de Claude Code), o automáticamente tras 14 días sin uso. |

**squeeze-Auditor** es la app que abres desde el menú Inicio o Aplicaciones: un panel con el uso real de Claude Code junto a lo que squeeze dejó fuera, modos de ahorro (Fuerte, Equilibrado, Ligero), todos los ajustes, el diagnóstico de coste fijo, tus notas de traspaso y las conexiones con Claude Code y la app de escritorio.

![Panel de squeeze-Auditor](../v0.2-Auditor/v0.2.0_squeeze-auditor/docs/images/auditor-en-dashboard.png)

Documentación completa de v0.2: [v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.es.md](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.es.md).

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

Documentación completa de la v0.1: [v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.es.md](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.es.md).

## Instalación

**Instalador (recomendado).** Desde la [última versión](https://github.com/jongwooish/claude-squeeze/releases/latest):

| Sistema | Archivo |
|---|---|
| Windows 10/11 | `squeeze-Auditor-0.2.0-setup-x64.exe` (ARM: `-setup-arm64.exe`) |
| macOS | `squeeze-Auditor-0.2.0-macos-arm64.pkg` (Intel: `-macos-x86_64.pkg`) |
| Linux (Debian, Ubuntu) | `squeeze-auditor_0.2.0_amd64.deb` (ARM: `_arm64.deb`) |

Los instaladores aún no están firmados. Windows SmartScreen: **Más información → Ejecutar de todas formas**. macOS: clic derecho → **Abrir**. Detalles: [instalación de v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.es.md#instalación).

**Terminal:**

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# Desde el código (cualquier sistema con Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

Cada archivo de la versión aparece en `checksums.txt` (SHA-256) y los scripts de instalación lo comprueban. `SQUEEZE_VERSION=v0.1.0` fija una versión; `SQUEEZE_INSTALL_DIR` cambia la carpeta.

## Inicio rápido

1. Abre **squeeze-Auditor** y pulsa **Conectar Claude Code** (todos los proyectos). ¿Usas la app de escritorio? Conéctala en **Conexiones** y reiníciala.
2. Usa Claude Code como siempre.
3. Vuelve al panel para ver qué se dejó fuera. Cambia cuánto ahorra squeeze en **Modos de ahorro** o **Ajustes**.

Desde una terminal:

```bash
squeeze setup      # conectar Claude Code (y la app de escritorio, si está instalada)
claude             # usa Claude Code como siempre
squeeze auditor    # abrir la ventana
squeeze setup --remove
```

Conectar solo cambia `~/.claude/settings.json` (y `claude_desktop_config.json` para la app de escritorio). Los demás ajustes y hooks se conservan, y antes se copia el original en `~/.squeeze/backups/`. El `squeeze init` por proyecto de v0.1 sigue funcionando.

## Uso

| Comando | Qué hace |
|---|---|
| `squeeze auditor` | Abre la ventana de squeeze-Auditor |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Conecta o desconecta Claude Code (todos los proyectos) y la app de escritorio |
| `squeeze doctor [--dir D] [--desktop] [--json]` | Lo que se envía con cada mensaje (coste fijo) |
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Instala hooks solo en un proyecto (como en v0.1) |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Quita los hooks y la nota de un proyecto; `--purge` borra también `.squeeze/` |
| `squeeze run -- <comando>` | Ejecuta un comando; resumen corto si la salida es larga |
| `<comando> 2>&1 \| squeeze compress` | Comprime la salida recibida por tubería |
| `squeeze clip [--print]` | Comprime el portapapeles (para claude.ai) |
| `squeeze report [--since 7d] [--global] [--json]` | Muestra el ahorro |
| `squeeze bench <dir> [--markdown]` | Reproduce archivos `*.log` y mide |
| `squeeze mcp` | El servidor MCP que ejecuta la app de escritorio |
| `squeeze version` | Muestra la versión |

Antepón `SQUEEZE_SKIP=1` a un comando para obtener su salida original una vez, o pon `SQUEEZE_SKIP` en un mensaje para pasar la pausa por presupuesto. Ajustes: [configuración de v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.es.md#configuración).

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
- [x] **v0.2 squeeze-Auditor**: bloqueos de lectura repetida, presupuesto, traspaso y coste fijo; app instalable con panel y modos de ahorro; herramientas para la app de escritorio de Claude
- [ ] **v0.3**: claude.ai en el navegador y las apps móviles, con un enfoque nuevo para los problemas de tokens que más se mencionan
- [ ] Instaladores firmados
- [ ] Benchmark A/B real con Claude Code (mismas tareas, squeeze activado y desactivado)
- [ ] v1.0: comandos y configuración estables

## FAQ

**P. ¿Puede Claude perderse algo importante?**
R. En todos los registros de prueba se conservaron todos los mensajes de error y advertencia distintos. Si el resumen no basta, la última línea da la ruta del log completo. Al bloquear una relectura, squeeze también le dice a Claude cómo volver a leer el archivo si de verdad lo necesita.

**P. ¿squeeze envía mis datos a algún sitio?**
R. No. No tiene código de red; la ventana de squeeze-Auditor es una página en `127.0.0.1` que solo tu ordenador puede abrir.

**P. ¿Puede romper mi sesión de Claude Code?**
R. Los hooks fallan en abierto: una entrada rara, un archivo inexistente o un fallo interno no producen salida y Claude Code sigue con normalidad. squeeze nunca aprueba una herramienta por ti.

**P. ¿Ahorra tokens en los chats normales de claude.ai?**
R. Solo cuando pegas texto largo después de ejecutar `squeeze clip`. El navegador y las apps móviles son el foco de v0.3.

**P. ¿El panel muestra el límite de mi plan?**
R. No. Lee los propios registros de Claude Code en este ordenador. No hay una forma oficial de que otros programas lean el uso del plan.

**P. ¿Por qué la versión 0.x?**
R. Mientras la versión principal sea 0, los nombres de comandos y la configuración pueden cambiar. La v1.0 significará que son estables.

## Estructura del repositorio

```
claude-squeeze/
├── README/                      ← resumen en 6 idiomas (README.md = inglés)
├── CHANGELOG/                   ← CHANGELOG en 6 idiomas
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   resumen de la versión
│   └── v0.1.0_squeeze-sentinel/   ← v0.1.0 tal como se publicó (congelada)
├── v0.2-Auditor/                ← v0.2 squeeze-Auditor
│   ├── README.md                   resumen de la versión
│   └── v0.2.0_squeeze-auditor/    ← v0.2: el programa squeeze actual (Rust, sin dependencias)
│       ├── README.md               documentación completa de v0.2 (inglés)
│       ├── README/                 traducciones (5 idiomas)
│       ├── src/                    motor, hooks, bloqueos, servidor de squeeze-Auditor
│       ├── ui/                     la ventana de squeeze-Auditor (un archivo HTML)
│       ├── packaging/              instaladores para Windows, macOS y Linux, icono
│       ├── docs/                   benchmark, guía de usuario (coreano), capturas
│       └── testdata/fixtures/      logs usados por los tests y el benchmark
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock (workspace de Rust)
├── .github/                     ← formularios de issues, CI, publicación, README de portada
├── CLAUDE.md
└── LICENSE
```

## Contribuir

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
```

Se agradecen nuevos logs de prueba: añade un `*.log` real (sin secretos ni rutas personales) a `v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures/`. Los tests fallan si se pierde cualquier error o advertencia distinto. Errores e ideas: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).

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
