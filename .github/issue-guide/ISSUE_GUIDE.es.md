# 📌 Guía para issues

[English](ISSUE_GUIDE.md) | [한국어](ISSUE_GUIDE.ko.md) | [日本語](ISSUE_GUIDE.ja.md) | [简体中文](ISSUE_GUIDE.zh-CN.md) | **Español** | [Français](ISSUE_GUIDE.fr.md)

¡Hola, y gracias por pasarte! 👋
claude-squeeze mejora gracias a que la gente nos cuenta qué falló y qué le gustaría. Cada aporte cuenta, aunque sea de una sola línea. Esta página explica cómo escribir un issue que se resuelva rápido.

### Qué puedes publicar aquí

- 🐞 **Algo se rompió**: squeeze falló, mostró algo raro o una sesión de Claude Code se comportó de forma extraña
- ✂️ **La compresión no fue bien**: desapareció una línea importante, se conservó demasiado o un comando se redirigió (o no)
- 📦 **Problemas de instalación, actualización o versión**: falló el instalador, faltaba un archivo de la versión o algo cambió tras actualizar
- 💡 **Ideas y mejoras**: funciones nuevas, mejores valores por defecto y lo que te gustaría ver en la **v1.0**
- ❓ **Preguntas**: ¿no sabes si es un error? Pregunta igualmente. A nadie le molestará.

### Antes de publicar (2 minutos)

1. **Comprueba tu versión**: ejecuta `squeeze version`. Si no es la [última versión](https://github.com/jongwooish/claude-squeeze/releases/latest), actualiza primero. Puede que ya esté arreglado.
2. **Busca issues existentes**: quizá alguien ya lo informó. Si es así, añade un 👍 o un comentario con tus detalles en lugar de abrir uno nuevo.
3. **Echa un vistazo a las [FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/README/README.es.md#faq)**: allí están las preguntas comunes.

### Elige el formulario adecuado

Pulsa **New issue** y elige:

| Si… | Elige |
|---|---|
| squeeze falló, dio una salida incorrecta o rompió una sesión | **Bug report / 버그 신고** |
| se perdió una línea importante o un comando se detectó (o no) | **Compression problem / 압축 문제** |
| el problema es instalar, actualizar o una versión concreta | **Install / update / version problem / 설치·업데이트·버전 문제** |
| tienes una idea o un deseo para la v1.0 | **Feature idea / 기능 제안** |

¿Dudas? Elige el más parecido. Lo moveremos si hace falta.

### Qué hace que un informe sea genial

- **Tu entorno**: versión de squeeze, sistema operativo y shell (en Windows: Git Bash, WSL o PowerShell)
- **El comando exacto** que ejecutaste tú o Claude
- **Lo que esperabas** y **lo que pasó realmente**
- **La salida como texto**, no como captura (las capturas son bienvenidas como extra)
- **Pasos para reproducirlo**, lo más cortos posible

Comandos útiles para reunir información:

```bash
squeeze version            # tu versión
squeeze report --json      # lo que squeeze dejó fuera del contexto recientemente
squeeze doctor --json      # lo que se envía con cada mensaje (v0.2)
SQUEEZE_SKIP=1 <comando>   # la salida original, para compararla con el resumen
```

Cuando squeeze comprime una salida, la última línea indica dónde se guardó el log completo (`.squeeze/raw/...`). Adjuntar ese archivo (tras limpiarlo, ver abajo) ayuda muchísimo con los problemas de compresión.

### 🔒 Protege tus secretos

Los logs pueden contener cosas que no quieres publicar. Antes de pegar, quita o reemplaza:

- Claves de API, tokens, contraseñas, cookies
- Nombres de host internos, direcciones IP, nombres de empresa
- Rutas personales como `C:\Users\tu-nombre\...` o `/home/tu-nombre/...` (cámbialas por `~/`)

¿Encontraste un **problema de seguridad**? No lo publiques. Sigue [SECURITY.md](../SECURITY.md) para informarlo en privado.

### Qué pasa después

- Leemos cada issue y le ponemos una etiqueta (`bug`, `enhancement`, `question`, …).
- Es un proyecto personal de código abierto, así que la respuesta puede tardar unos días. Gracias por tu paciencia.
- Puede que te hagamos una pregunta. Responderla suele ser el camino más rápido hacia la solución.
- Las correcciones y novedades se anotan en el [CHANGELOG](../../CHANGELOG/CHANGELOG.es.md) y se anuncian en [Releases](https://github.com/jongwooish/claude-squeeze/releases). Pulsa **Watch → Custom → Releases** para recibir avisos.

### Unas pocas normas amables

- Un problema por issue, para poder seguir y cerrar cada uno.
- Sé amable. Todos aquí quieren ayudar.
- Cualquiera de los seis idiomas sirve. En inglés o coreano la respuesta es más rápida.

¡Gracias por ayudar a mejorar claude-squeeze! 🙏
