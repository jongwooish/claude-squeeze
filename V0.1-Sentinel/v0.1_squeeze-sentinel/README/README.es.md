# v0.1 Sentinel

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | **Español** | [Français](README.fr.md)

> 💬 **Se agradecen comentarios.** Mejoras, el lanzamiento de la v1.0, actualizaciones, errores de instalación o cualquier otro problema: [abre un issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 idiomas).

← [Volver a squeeze](../../../readme/README.es.md)

Sentinel hace guardia entre Claude Code y tu shell. Cuando Claude va a ejecutar un comando que inunda el contexto (instalaciones, builds, tests, logs) o a leer de una vez un archivo enorme, Sentinel lo redirige a una versión corta. Claude sigue recibiendo cada error y advertencia.

Un solo binario, escrito en Rust, **cero dependencias**. Funciona en macOS, Linux y Windows.

**Requisitos:** ninguno. Este es el programa base `squeeze`. Las versiones posteriores se construyen sobre él: v0.3 Conduit añade un comando a este mismo programa (actualiza para tenerlo), mientras que v0.2 Interceptor es una extensión de navegador aparte que funciona con o sin él. Consulta [cómo encajan las versiones](../../../readme/README.es.md#cómo-encajan-las-versiones).

## Efecto

Reproducción de 7 logs de prueba, de 83,325 a 5,128 tokens estimados (**−93.8%**), con cada error y advertencia distinto conservado. Detalles y método: [docs/bench.md](../docs/bench.md) (inglés).

Lo que Claude ve tras `squeeze run -- pytest -v` (217 líneas → 39). Es la salida real del log `pytest-fail.log`:

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
_______________________________ test_bulk_order ________________________________

    def test_bulk_order():
        orders = {"apple": 3, "pear": 5}
>       assert sum(orders.values()) == 9, "bulk order total mismatch"
E       AssertionError: bulk order total mismatch
E       assert 8 == 9
E        +  where 8 = sum(dict_values([3, 5]))
E        +    where dict_values([3, 5]) = <built-in method values of dict object at 0x7f4c5d1f9900>()
E        +      where <built-in method values of dict object at 0x7f4c5d1f9900> = {'apple': 3, 'pear': 5}.values

test_inventory.py:12: AssertionError
_______________________________ test_missing_sku _______________________________

    def test_missing_sku():
        stock = {"A1": 3}
>       assert stock["B7"] == 0
               ^^^^^^^^^^^
E       KeyError: 'B7'

test_inventory.py:15: KeyError
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

## Funciones

- **Guardia de Bash**: los comandos ruidosos (`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log` y más) se redirigen a `squeeze run -- <comando>`.
- **Guardia de Read**: leer un archivo de más de 400 líneas sin `offset`/`limit` se redirige. Claude recibe un índice con números de línea y elige un rango.
- **Compresión inteligente**: conserva las primeras 8 y las últimas 20 líneas, cada error y advertencia con su contexto y las trazas de pila. Los mensajes repetidos se agrupan con un contador. Los errores van antes que las advertencias.
- **Código de salida conservado**: `squeeze run` termina con el código del propio comando.
- **Logs completos en disco**: `.squeeze/raw/`, se guardan los 50 más recientes, legibles solo por ti (0600). `.squeeze/` se añade a `.gitignore`.
- **`squeeze clip`** para claude.ai: comprime el portapapeles antes de pegar un log en un chat.
- **`squeeze report`**: lo que habría entrado en el contexto, lo que entró y los mayores ahorros.
- **Seguro por diseño**: los hooks nunca aprueban una llamada a herramienta; ante cualquier error se quedan en silencio (fail open); `squeeze uninstall` elimina todo lo que añadió.

## Instalación

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.ps1 | iex
# Desde el código fuente
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

Los instaladores verifican el `checksums.txt` de la versión antes de instalar. `SQUEEZE_VERSION=v0.1.0` fija una versión; `SQUEEZE_INSTALL_DIR` cambia la carpeta.

## Inicio rápido

```bash
cd ~/my-project
squeeze init
claude
```

`squeeze init` cambia cuatro cosas y `squeeze uninstall` las deshace:

| Archivo | Cambio |
|---|---|
| `.claude/settings.json` | Añade dos hooks `PreToolUse` (`Bash`, `Read`). Se conservan tus otros ajustes y hooks; antes se escribe una copia `.squeeze.bak`. |
| `.squeeze/config.json` | Configuración por defecto (ver abajo). |
| `.gitignore` | Añade `.squeeze/`, porque los logs originales pueden contener secretos. |
| `CLAUDE.md` | Añade una nota de 4 líneas entre marcadores `<!-- squeeze:start -->`. |

## Uso

| Comando | Qué hace |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Instalar en un proyecto. Usa `--bin` con una ruta absoluta si `squeeze` no está en el `PATH`. |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Quitar los hooks y la nota de CLAUDE.md. `--purge` también borra `.squeeze/`. |
| `squeeze run -- <comando>` | Ejecutar un comando; mostrar un resumen corto si la salida es larga. |
| `<comando> 2>&1 \| squeeze compress` | Comprimir salida por tubería. |
| `squeeze clip [--print]` | Sustituir el portapapeles por la versión comprimida (`--print` solo la muestra). |
| `squeeze report [--since 7d] [--global] [--json] [--top N]` | Ahorro de este proyecto. `--global` muestra el ahorro de `squeeze clip`. |
| `squeeze bench <dir> [--markdown]` | Reproducir archivos `*.log` y medir el ahorro. |
| `squeeze hook pre-bash` / `pre-read` | Puntos de entrada de los hooks que llama Claude Code. |
| `squeeze version` | Mostrar la versión. |

Si de verdad hace falta la salida original de un comando, Claude (o tú) puede anteponer `SQUEEZE_SKIP=1`.

## Configuración

`.squeeze/config.json` (todas las claves son opcionales):

| Clave | Por defecto | Significado |
|---|---|---|
| `compress.max_lines` / `max_bytes` | `80` / `8000` | La salida que no supera ambos límites se deja igual. |
| `compress.head_lines` / `tail_lines` | `8` / `20` | Líneas que siempre se conservan del inicio / final. |
| `compress.context` | `2` | Líneas conservadas alrededor de cada error o advertencia. |
| `compress.budget` | `120` | Límite aproximado de líneas conservadas. |
| `guard_bash` / `guard_read` | `true` / `true` | Activar o desactivar cada guardia. |
| `read_max_lines` | `400` | Los archivos más largos se redirigen a una lectura por rango. |
| `extra_noisy` | `[]` | Prefijos de comandos que se consideran ruidosos, p. ej. `"./deploy.sh"`. |
| `quiet` | `[]` | Prefijos de comandos que la guardia nunca toca, p. ej. `"make docs"`. |
| `keep_raw` | `50` | Logs originales guardados en `.squeeze/raw/`. |

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
        ▼
salida completa ──▶ .squeeze/raw/<hora>.log
salida corta ─────▶ Claude (errores, advertencias, inicio, final y la línea con la ruta del log)
```

¿Por qué "denegar con una pista" y no reescribir el comando en silencio? Un hook que reescribe un comando también tiene que aprobarlo, y eso se saltaría tus confirmaciones de permisos. squeeze nunca lo hace.

Más: [docs/guide/how-it-works.md](../docs/guide/how-it-works.md) (coreano).

## Dónde se aplica

- **Claude Code**: automático, mediante hooks.
- **claude.ai**: solo cuando pegas texto largo usando `squeeze clip`. El chat normal no se ve afectado; squeeze no puede ver ni cambiar tus chats.
- **Otras herramientas de terminal**: pasa la salida por `squeeze compress`.

## FAQ

**P. ¿Puede Claude perderse algo importante?**
R. En todos los logs de prueba se conservaron todos los mensajes de error y advertencia distintos. Si el resumen no basta, la última línea da la ruta del log completo y Claude puede leerlo por rangos.

**P. ¿squeeze envía mis datos a algún sitio?**
R. No. No tiene código de red. Los logs se quedan en `.squeeze/` en tu máquina.

**P. ¿Puede romper mi sesión?**
R. Los hooks se apartan ante fallos: una entrada extraña, un archivo inexistente o un panic no producen salida, y Claude Code sigue con normalidad.

**P. Claude se topa una y otra vez con un archivo grande y yo lo necesito entero.**
R. La pista le dice a Claude que vuelva a llamar a Read con `limit=<líneas>`, y así pasa. O desactívalo con `"guard_read": false`.

**P. ¿Ahorra tokens en los chats normales de claude.ai?**
R. Solo cuando pegas texto largo tras ejecutar `squeeze clip`.

## Comentarios

Deja mejoras y problemas en [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose): ideas para la v1.0, errores de actualización o del instalador, compresión que perdió algo o cualquier otro fallo. Elige un formulario (error, problema de compresión, problema de instalación/actualización/versión, idea). Preguntas y hoja de ruta en [Discussions](https://github.com/jongwooish/claude-squeeze/discussions). Cualquiera de los seis idiomas sirve.

## Contribuir

```bash
cargo test                       # tests unitarios, incluidas las comprobaciones de logs
cargo clippy -- -D warnings
cargo run -- bench V0.1-Sentinel/v0.1_squeeze-sentinel/testdata/fixtures
```

Se agradecen nuevos logs de prueba: añade un `*.log` real (quita antes secretos y rutas personales) a `testdata/fixtures/`. Los tests fallan si se pierde cualquier error o advertencia distinto.

Problemas e ideas: [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).
