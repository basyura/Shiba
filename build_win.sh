#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

make target/release/shiba.exe
ls -lh target/release/shiba.exe
