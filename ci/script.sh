#!/usr/bin/env bash

set -euxo pipefail

main() {
    cargo check --target "$TARGET"
    cargo build --target "$TARGET" --examples
}

main
