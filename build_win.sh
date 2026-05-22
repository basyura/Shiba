#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

# Prefer rustup's MSVC toolchain over MSYS2's Cygwin rustc if available
MSVC_TOOLCHAIN="$HOME/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin"
if [[ -x "$MSVC_TOOLCHAIN/cargo.exe" ]]; then
    export PATH="$MSVC_TOOLCHAIN:$PATH"
fi

make target/release/shiba.exe
ls -lh target/release/shiba.exe
