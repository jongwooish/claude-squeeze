# Registro de cambios

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | **Español** | [Français](CHANGELOG.fr.md)

Aquí se recogen los cambios importantes. Las versiones siguen [Versionado Semántico](https://semver.org/lang/es/). Mientras la versión mayor sea 0, los comandos y la configuración pueden cambiar.

## [0.1.0] - 2026-11-03 — Sentinel

Primera versión. Código en [`v0.1-Sentinel/v0.1_squeeze-sentinel/`](../v0.1-Sentinel/v0.1_squeeze-sentinel/).

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
