#!/bin/bash

# This requires you to previously run `cargo install defmt-print`

# See https://ferroussystems.hackmd.io/@jonathanpallant/ryA1S6QDJx for a description of all the relevant QEMU machines
TARGET=""
ELF_BINARY=""

# very small argument parser
while [[ $# -gt 0 ]]; do
    case "$1" in
        --target)   TARGET="$2"; shift 2 ;;
        *)          ELF_BINARY="$1"; shift ;;
    esac
done

# default to the target cargo is currently building for
TARGET="${TARGET:-thumbv7em-none-eabihf}"

case "$TARGET" in
    thumbv6m-none-eabi)
        MACHINE="-cpu cortex-m3 -machine mps2-an385" ;;
    thumbv7em-none-eabihf)
        # All suitable for thumbv7em-none-eabihf
        MACHINE="-cpu cortex-m4 -machine mps2-an386" ;;
        # MACHINE="-cpu cortex-m7 -machine mps2-387" ;;
        # MACHINE="-cpu cortex-m7 -machine mps2-500"
    *)
        echo "Unsupported target: $TARGET" >&2
        exit 1 ;;
esac

LOG_FORMAT='{[{L}]%bold} {s} {({ff}:{l:1})%dimmed}'

echo "Running on '$MACHINE'..."
echo "------------------------------------------------------------------------"
qemu-system-arm $MACHINE -semihosting-config enable=on,target=native -nographic -kernel $ELF_BINARY | defmt-print -e $ELF_BINARY --log-format="$LOG_FORMAT"
echo "------------------------------------------------------------------------"
