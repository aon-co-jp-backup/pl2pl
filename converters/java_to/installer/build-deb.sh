#!/usr/bin/env bash
# Linux .deb を作る。使い方: build-deb.sh <java_to バイナリ> <版> <amd64|arm64> <出力ディレクトリ>
set -euo pipefail
bin="$1"; ver="$2"; arch="$3"; out="$4"
root="$(cd "$(dirname "$0")/.." && pwd)"
pkg="$(mktemp -d)/java-to_${ver}_${arch}"
mkdir -p "$pkg/DEBIAN" "$pkg/usr/bin" "$pkg/usr/share/doc/java-to" "$pkg/usr/share/applications"
install -m 0755 "$bin" "$pkg/usr/bin/java_to"
install -m 0644 "$root/README.md" "$root/LICENSE" "$pkg/usr/share/doc/java-to/"
cat > "$pkg/usr/share/applications/java_to.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Java_to
Comment=Java dialect auto-detection and conversion
Exec=java_to
Terminal=true
Categories=Development;
EOF
cat > "$pkg/DEBIAN/control" <<EOF
Package: java-to
Version: ${ver}
Section: devel
Priority: optional
Architecture: ${arch}
Depends: libc6, libwayland-client0
Recommends: xdg-desktop-portal
Maintainer: aon-co-jp <noreply@users.noreply.github.com>
Homepage: https://github.com/aon-co-jp/Java_to
Description: Java dialect auto-detection and conversion to Rust/RPoem/Python stacks
 Detects the Java dialect, standard and source format and converts to
 FastAPI, MFastAPI, Rust+Poem or Rust+RPoem projects.
EOF
mkdir -p "$out"
dpkg-deb --root-owner-group --build "$pkg" "$out/java-to_${ver}_${arch}.deb" >/dev/null
echo "$out/java-to_${ver}_${arch}.deb"
