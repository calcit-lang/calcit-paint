#!/usr/bin/env sh
set -eu

cargo build --release
mkdir -p dylibs

case "$(uname -s)" in
  Darwin) artifact=target/release/libcalcit_paint.dylib ;;
  Linux) artifact=target/release/libcalcit_paint.so ;;
  MINGW*|MSYS*|CYGWIN*) artifact=target/release/calcit_paint.dll ;;
  *) echo "unsupported platform: $(uname -s)" >&2; exit 1 ;;
esac

cp -v "$artifact" dylibs/
