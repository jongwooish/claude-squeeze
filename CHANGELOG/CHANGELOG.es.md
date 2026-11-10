# Registro de cambios

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | **Español** | [Français](CHANGELOG.fr.md)

Aquí se recogen los cambios importantes. Las versiones siguen [Versionado Semántico](https://semver.org/lang/es/). Mientras la versión mayor sea 0, los comandos y la configuración pueden cambiar.

## [0.2.0] - 2026-11-04 — squeeze-Auditor

v0.2 mira la sesión completa, no una salida cada vez. Código en [`v0.2-Auditor/v0.2.0_squeeze-auditor/`](../v0.2-Auditor/v0.2.0_squeeze-auditor/). Incluye todo lo de 0.1.0.

### Añadido

- Cuatro bloqueos de sesión (Claude Code):
  - **Lectura repetida**: se rechaza un `Read` completo de un archivo ya leído en esta sesión y sin cambios. Se permite de nuevo cuando el archivo cambia o el contexto se compacta.
  - **Pausa por presupuesto** (`UserPromptSubmit`): cuando la sesión supera el presupuesto (150K tokens por defecto), el mensaje se retiene una vez con la sugerencia de `/compact` o `/clear`. Enviarlo de nuevo lo deja pasar.
  - **Traspaso** (`Stop` + `SessionStart`): cuando la sesión supera el tamaño de traspaso (120K por defecto), Claude escribe una nota en `.squeeze/handoff/`; tras `/clear`, la nueva sesión empieza desde la nota. El modo `remind` solo sugiere `/compact`.
  - **Coste fijo**: mide los archivos `CLAUDE.md` e importaciones, skills, agentes y listas de herramientas MCP enviados con cada mensaje. Apaga servidores MCP por proyecto con el interruptor de Claude Code, o automáticamente tras N días sin uso (desactivado por defecto).
- App **squeeze-Auditor** (`squeeze-auditor`, `squeeze auditor`): panel con el uso real de Claude Code según sus registros y lo que squeeze dejó fuera, la ventana de 5 horas actual, modos de ahorro (Fuerte, Equilibrado, Ligero, Personalizado), todos los ajustes, diagnóstico de coste fijo con Medir y Encender/Apagar, notas de traspaso, conexiones. Solo local (`127.0.0.1`, clave de un solo uso). Inglés y coreano.
- Soporte para la **app de escritorio de Claude**: `squeeze mcp`, un servidor MCP con `read_log`, `tail_log`, `search_log`, `list_logs`, `save_handoff`, `load_handoff`, limitado a las carpetas que permitas.
- `squeeze setup [--claude-code] [--desktop] [--remove]`: conectar una vez para todos los proyectos (`~/.claude/settings.json`) y la app de escritorio, con copias en `~/.squeeze/backups/`.
- `squeeze doctor`: coste fijo por mensaje en la terminal.
- Instaladores: Windows Setup (x64, ARM64, por usuario, icono en el menú Inicio, desinstalador que desconecta), `.pkg` de macOS con `squeeze-Auditor.app`, `.deb` de Linux con entrada en el menú. Aún sin firma de código.
- Configuración global `~/.squeeze/config.json` (el `.squeeze/config.json` de un proyecto sigue teniendo prioridad) y un registro global `~/.squeeze/ledger.jsonl`.

### Cambiado

- Los scripts de instalación también instalan `squeeze-auditor`.
- El título de cada versión publicada sale ahora del encabezado del CHANGELOG. La CI también construye los instaladores y prueba la instantánea congelada de v0.1.0.
- El crate congelado de v0.1.0 se llama `squeeze-sentinel` (el programa sigue siendo `squeeze`), así que `cargo install --git … squeeze` construye v0.2.

### Previsto

- v0.3: claude.ai en el navegador y las apps móviles, con un enfoque nuevo.

## [0.1.0] - 2026-11-03 — Sentinel

Primera versión. Código en [`v0.1-Sentinel/v0.1.0_squeeze-sentinel/`](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/).

### Añadido

- Hooks `PreToolUse` de Claude Code:
  - **Guardia de Bash**: redirige los comandos ruidosos (instalaciones, builds, tests, logs) a `squeeze run`. No toca los comandos ya acotados (`| tail`, `-n 50`, redirecciones, `--quiet`).
  - **Guardia de Read**: redirige la lectura completa de archivos de más de 400 líneas a una lectura por rango, con un índice.
  - Los hooks nunca aprueban una llamada a herramienta y se apartan ante cualquier error (fail open).
- `squeeze run -- <comando>`: ejecutar, comprimir la salida larga, guardar el log completo, conservar el código de salida.
- `squeeze compress`: comprimir salida por tubería.
- `squeeze clip`: comprimir el portapapeles para pegarlo en claude.ai (macOS, Windows, Linux).
- `squeeze report`: ahorro por proyecto (`--global` para clip, `--json`, `--since`).
- `squeeze bench`: reproducir logs y medir el ahorro y los mensajes conservados.
- `squeeze init` / `squeeze uninstall`: instalar y quitar limpiamente hooks, configuración, entrada en `.gitignore` y nota en `CLAUDE.md`.
- Compresor: conserva inicio y final, cada error y advertencia distinto con contexto y las trazas de pila; agrupa repeticiones; prioriza errores sobre advertencias.
- Siete logs de benchmark: 93.8% menos tokens estimados, 50/50 mensajes distintos conservados.
- Instaladores para macOS/Linux (`install.sh`) y Windows (`install.ps1`) con verificación SHA-256.
- READMEs en inglés, coreano, japonés, chino simplificado, español y francés.

### Planeado

- v0.2 Interceptor: extensión de navegador para pegar en claude.ai.
- v0.3 Conduit: servidor MCP para la app de escritorio de Claude.
- Benchmark A/B en vivo con Claude Code.
