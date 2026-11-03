#!/usr/bin/env bash
# Install squeeze from the latest GitHub release.
#   curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/install.sh | bash
# Pin a version:  ... | SQUEEZE_VERSION=v0.1.0 bash
# Install dir:    ... | SQUEEZE_INSTALL_DIR=$HOME/.local/bin bash
set -euo pipefail

REPO="jongwooish/claude-squeeze"
VERSION="${SQUEEZE_VERSION:-latest}"
DIR="${SQUEEZE_INSTALL_DIR:-/usr/local/bin}"

say() { printf 'squeeze-install: %s\n' "$*"; }
die() { say "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || die "missing required tool: $1"; }

need curl; need tar; need uname; need mktemp

os="$(uname -s)"; arch="$(uname -m)"
case "$arch" in
  x86_64|amd64) arch=x86_64 ;;
  arm64|aarch64) arch=aarch64 ;;
  *) die "unsupported CPU: $arch" ;;
esac
case "$os" in
  Linux)  target="$arch-unknown-linux-musl" ;;
  Darwin) target="$arch-apple-darwin" ;;
  *) die "unsupported OS: $os (on Windows use install.ps1)" ;;
esac

if [ "$VERSION" = "latest" ]; then
  base="https://github.com/$REPO/releases/latest/download"
else
  base="https://github.com/$REPO/releases/download/$VERSION"
fi
asset="squeeze-$target.tar.gz"

tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
say "downloading $asset ($VERSION)"
curl -fsSL "$base/$asset" -o "$tmp/$asset" || die "download failed: $base/$asset"
curl -fsSL "$base/checksums.txt" -o "$tmp/checksums.txt" || die "could not download checksums.txt"

expected="$(awk -v a="$asset" '$2==a {print $1}' "$tmp/checksums.txt")"
[ -n "$expected" ] || die "$asset not listed in checksums.txt"
if command -v sha256sum >/dev/null 2>&1; then actual="$(sha256sum "$tmp/$asset" | awk '{print $1}')"
else need shasum; actual="$(shasum -a 256 "$tmp/$asset" | awk '{print $1}')"; fi
[ "$expected" = "$actual" ] || die "checksum mismatch for $asset"
say "checksum ok"

tar xzf "$tmp/$asset" -C "$tmp"
mkdir -p "$DIR" 2>/dev/null || true
if [ -w "$DIR" ]; then install -m 0755 "$tmp/squeeze" "$DIR/squeeze"
else say "using sudo to write $DIR"; sudo install -m 0755 "$tmp/squeeze" "$DIR/squeeze"; fi

say "installed $("$DIR/squeeze" version) to $DIR/squeeze"
case ":$PATH:" in *":$DIR:"*) ;; *) say "note: $DIR is not on your PATH" ;; esac
say "next: cd your-project && squeeze init"
