#!/bin/bash
set -e
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $DIR

echo "Building release binary"
cargo build --release

echo "Building debian package"
export PACKAGE_DIR="deb"
export BINARY_NAME="netspeed"
export ARCH="amd64"
export VERSION="0.1.0"
export MAINTAINER="Divyanshu Agrawal <agrawal-d@outlook.com>"
export CONTROL_FILE="$PACKAGE_DIR/DEBIAN/control"

rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR/DEBIAN"
mkdir -p "$PACKAGE_DIR/usr/bin"
mkdir -p "$PACKAGE_DIR/opt/$BINARY_NAME"
mkdir -p "$PACKAGE_DIR/usr/share/applications"
mkdir -p "$PACKAGE_DIR/usr/share/man/man1/"
cp icon.png $PACKAGE_DIR/opt/$BINARY_NAME
strip target/release/ui
cp target/release/ui "$PACKAGE_DIR/usr/bin/$BINARY_NAME"
cp netspeed.desktop "$PACKAGE_DIR/usr/share/applications"
cp netspeed.1 "$PACKAGE_DIR/usr/share/man/man1/"

touch "$CONTROL_FILE"
echo "Package: $BINARY_NAME" >> "$CONTROL_FILE"
echo "Version: $VERSION" >> "$CONTROL_FILE"
echo "Section: base" >> "$CONTROL_FILE"
echo "Priority: optional" >> "$CONTROL_FILE"
echo "Architecture: $ARCH" >> "$CONTROL_FILE"
echo "Maintainer: $MAINTAINER" >> "$CONTROL_FILE"
echo "Description: Monitor statistics like upload/download speeds and other stats of your networks in a GUI" >> "$CONTROL_FILE"

dpkg-deb --build "$PACKAGE_DIR" "$PACKAGE_DIR/$BINARY_NAME-$VERSION-$ARCH.deb"

echo "Done building debian package"