#!/usr/bin/env bash
# Build squeeze-Auditor-<version>-macos-<arch>.pkg (unsigned).
#   build-pkg.sh <version> <arch: arm64|x86_64> <folder with squeeze and squeeze-auditor> <out dir>
# The .pkg installs /Applications/squeeze-Auditor.app and links
# /usr/local/bin/squeeze to the program inside it.
set -euo pipefail
ver="$1"; arch="$2"; bin="$3"; out="$4"
here="$(cd "$(dirname "$0")" && pwd)"
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
app="$work/root/Applications/squeeze-Auditor.app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources" "$work/scripts" "$out"
install -m 0755 "$bin/squeeze" "$bin/squeeze-auditor" "$app/Contents/MacOS/"
# build the .icns from the PNG with macOS's own tools
set_dir="$work/squeeze-auditor.iconset"; mkdir -p "$set_dir"
for s in 16 32 128 256 512; do
  sips -z $s $s "$here/../icon/icon.png" --out "$set_dir/icon_${s}x${s}.png" >/dev/null
  d=$((s * 2)); [ $d -le 512 ] && sips -z $d $d "$here/../icon/icon.png" --out "$set_dir/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$set_dir" -o "$app/Contents/Resources/squeeze-auditor.icns"
sed "s/__VERSION__/$ver/g" "$here/Info.plist" > "$app/Contents/Info.plist"
cat > "$work/scripts/postinstall" <<'SH'
#!/bin/sh
mkdir -p /usr/local/bin
ln -sf "/Applications/squeeze-Auditor.app/Contents/MacOS/squeeze" /usr/local/bin/squeeze
exit 0
SH
chmod +x "$work/scripts/postinstall"
pkgbuild --root "$work/root" --scripts "$work/scripts" \
  --identifier io.github.jongwooish.squeeze-auditor --version "$ver" \
  --install-location / "$out/squeeze-Auditor-$ver-macos-$arch.pkg"
