#!/usr/bin/env sh
set -e

if ! command -v numfmt > /dev/null 2>&1
then
    echo "numfmt is needed for human-readable sizes." >&2
    echo "please install https://command-not-found.com/numfmt" >&2
    alias numfmt=true
fi

if ! command -v avrdude > /dev/null 2>&1
then
    echo "required avrdude could not be found!" >&2
    echo "please install https://command-not-found.com/avrdude" >&2
    exit 1
fi

SERIAL_PORT="/dev/ttyACM0"

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <application.elf>" >&2
    exit 1
fi

# if [ "$#" -lt 1 ]; then
#     echo "$0: no ELF file given" >&2
#     exit 1
# fi
# TARGET=="$1"
TARGET=target/avr-unknown-gnu-atmega328/release/opencat.elf
NAME="$(basename "$TARGET")"
SIZE_TEXT="$(avr-size "$TARGET" | tail -1 | cut -f1)"
SIZE_DATA="$(avr-size "$TARGET" | tail -1 | cut -f2)"
SIZE_BSS="$(avr-size "$TARGET" | tail -1 | cut -f3)"

printf "\n"
printf "Program:             %s\n" "$NAME"
printf "Size:\n"
printf "   .text   %s (exact: %d)\n" "$(numfmt --to=si --padding=9 "$SIZE_TEXT")" "$SIZE_TEXT"
printf "   .data   %s (exact: %d)\n" "$(numfmt --to=si --padding=9 "$SIZE_DATA")" "$SIZE_DATA"
printf "   .bss    %s (exact: %d)\n" "$(numfmt --to=si --padding=9 "$SIZE_BSS")" "$SIZE_BSS"
printf "\n"
printf "Attempting to flash ...\n"
printf "\n"

avrdude -q  -patmega328p -carduino -P"${SERIAL_PORT}" -D "-Uflash:w:$TARGET:e"
