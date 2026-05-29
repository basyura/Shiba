#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

APP_DIR="Shiba.app"
APP_BINARY="$APP_DIR/Contents/MacOS/shiba"
TARGET_BINARY="target/aarch64-apple-darwin/release/shiba"

npm run release
cargo build --release --target=aarch64-apple-darwin

rm -rf "$APP_DIR"
cp -R assets/Shiba.app "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
cp "$TARGET_BINARY" "$APP_BINARY"

ls -ld "$APP_DIR"
ls -lh "$APP_BINARY"
