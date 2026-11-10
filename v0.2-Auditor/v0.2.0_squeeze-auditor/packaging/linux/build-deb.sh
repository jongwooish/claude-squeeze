#!/usr/bin/env bash
# Build squeeze-auditor_<version>_<arch>.deb.
#   build-deb.sh <version> <arch: amd64|arm64> <folder with squeeze and squeeze-auditor> <out dir>
set -euo pipefail
ver="$1"; arch="$2"; bin="$3"; out="$4"
here="$(cd "$(dirname "$0")" && pwd)"
root="$(mktemp -d)"; trap 'rm -rf "$root"' EXIT
chmod 0755 "$root"
install -Dm0755 "$bin/squeeze" "$root/usr/bin/squeeze"
install -Dm0755 "$bin/squeeze-auditor" "$root/usr/bin/squeeze-auditor"
install -Dm0644 "$here/squeeze-auditor.desktop" "$root/usr/share/applications/squeeze-auditor.desktop"
install -Dm0644 "$here/../icon/icon.png" "$root/usr/share/icons/hicolor/512x512/apps/squeeze-auditor.png"
install -Dm0644 "$here/../icon/icon.svg" "$root/usr/share/icons/hicolor/scalable/apps/squeeze-auditor.svg"
install -Dm0644 "$here/../../LICENSE" "$root/usr/share/doc/squeeze-auditor/copyright"
mkdir -p "$root/DEBIAN" "$out"
cat > "$root/DEBIAN/control" <<CTL
Package: squeeze-auditor
Version: $ver
Architecture: $arch
Maintainer: claude-squeeze <https://github.com/jongwooish/claude-squeeze>
Homepage: https://github.com/jongwooish/claude-squeeze
Section: devel
Priority: optional
Description: Save Claude Code tokens and see where they go
 squeeze keeps long command output, repeated file reads, oversized sessions
 and idle MCP servers out of Claude Code's context. squeeze-Auditor shows
 real usage and what squeeze kept out, in a local window.
CTL
dpkg-deb --root-owner-group --build "$root" "$out/squeeze-auditor_${ver}_${arch}.deb"
