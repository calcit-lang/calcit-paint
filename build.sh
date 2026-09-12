#!/usr/bin/env sh
set -eu

cargo build --release
mkdir -p dylibs

case "$(uname -s)" in
  Darwin)
    artifact=target/release/libcalcit_paint.dylib
    dest=libcalcit_paint.dylib
    ;;
  Linux)
    artifact=target/release/libcalcit_paint.so
    dest=libcalcit_paint.so
    ;;
  MINGW*|MSYS*|CYGWIN*)
    # Rust omits the `lib` prefix for Windows cdylibs; install the name that
    # `calcit-paint.util/get-dylib-path` resolves (`libcalcit_paint.dll`).
    artifact=target/release/calcit_paint.dll
    dest=libcalcit_paint.dll
    ;;
  *) echo "unsupported platform: $(uname -s)" >&2; exit 1 ;;
esac

cp -v "$artifact" "dylibs/$dest"
