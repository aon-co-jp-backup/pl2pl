#!/usr/bin/env bash
# Linux .deb を作る。使い方: build-deb.sh <cobol_to バイナリ> <版> <amd64|arm64> <出力ディレクトリ>
set -euo pipefail
bin="$1"; ver="$2"; arch="$3"; out="$4"
root="$(cd "$(dirname "$0")/.." && pwd)"
pkg="$(mktemp -d)/cobol-to_${ver}_${arch}"
mkdir -p "$pkg/DEBIAN" "$pkg/usr/bin" "$pkg/usr/share/doc/cobol-to" "$pkg/usr/share/applications"
install -m 0755 "$bin" "$pkg/usr/bin/cobol_to"
install -m 0644 "$root/README.md" "$root/NOTIFY.md" "$root/LICENSE" "$pkg/usr/share/doc/cobol-to/"
cat > "$pkg/usr/share/applications/cobol_to.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=COBOL_to
Comment=COBOL dialect auto-detection and conversion
Exec=cobol_to
Terminal=true
Categories=Development;
EOF
cat > "$pkg/DEBIAN/control" <<EOF
Package: cobol-to
Version: ${ver}
Section: devel
Priority: optional
Architecture: ${arch}
Depends: libc6, libwayland-client0
Recommends: xdg-desktop-portal
Maintainer: aon-co-jp <noreply@users.noreply.github.com>
Homepage: https://github.com/aon-co-jp/COBOL_to
Description: COBOL dialect auto-detection and conversion to Rust/RPoem/Python stacks
 Detects the COBOL dialect, standard and source format and converts to
 FastAPI, MFastAPI, Rust+Poem or Rust+RPoem projects.
EOF
mkdir -p "$out"
dpkg-deb --root-owner-group --build "$pkg" "$out/cobol-to_${ver}_${arch}.deb" >/dev/null
echo "$out/cobol-to_${ver}_${arch}.deb"
