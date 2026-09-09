#!/usr/bin/env bash
set -euo pipefail

first_png="$(mktemp "${TMPDIR:-/tmp}/calcit-paint-art-first.XXXXXX")"
second_png="$(mktemp "${TMPDIR:-/tmp}/calcit-paint-art-second.XXXXXX")"
calcit_bin="${CALCIT_BIN:-calcit}"
trap 'rm -f "$first_png" "$second_png"' EXIT

"$calcit_bin" ./calcit.cirru eval --dep ./ 'ns creative-art.smoke $ :require
  calcit-paint.core :refer $ validate-scene
  calcit-paint.creative-art :refer $ build-art-scene export-art-frame!

let
    no-diagnostics $ []
    scene $ build-art-scene 17 0 |testing false
  do
    assert= no-diagnostics $ validate-scene scene
    export-art-frame! |'"$first_png"' 17 0
    export-art-frame! |'"$second_png"' 17 0
    , &unit'

python3 - "$first_png" "$second_png" <<'PY'
import hashlib
import struct
import sys
import zlib


def decode_rgba(path):
    with open(path, "rb") as stream:
        data = stream.read()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", "expected a PNG signature"

    offset = 8
    width = height = None
    compressed = bytearray()
    while offset < len(data):
        length = struct.unpack(">I", data[offset : offset + 4])[0]
        kind = data[offset + 4 : offset + 8]
        chunk = data[offset + 8 : offset + 8 + length]
        offset += 12 + length
        if kind == b"IHDR":
            width, height, depth, color, compression, filter_method, interlace = struct.unpack(
                ">IIBBBBB", chunk
            )
            assert (depth, color, compression, filter_method, interlace) == (
                8,
                6,
                0,
                0,
                0,
            ), "expected RGBA8888 PNG"
        elif kind == b"IDAT":
            compressed.extend(chunk)

    raw = zlib.decompress(compressed)
    stride = width * 4
    rows = []
    cursor = 0
    for row_index in range(height):
        filter_type = raw[cursor]
        cursor += 1
        row = bytearray(raw[cursor : cursor + stride])
        cursor += stride
        previous = rows[row_index - 1] if row_index else bytearray(stride)
        for index, value in enumerate(row):
            left = row[index - 4] if index >= 4 else 0
            up = previous[index]
            upper_left = previous[index - 4] if index >= 4 else 0
            if filter_type == 1:
                row[index] = (value + left) & 255
            elif filter_type == 2:
                row[index] = (value + up) & 255
            elif filter_type == 3:
                row[index] = (value + ((left + up) // 2)) & 255
            elif filter_type == 4:
                predictor = left + up - upper_left
                distances = (
                    abs(predictor - left),
                    abs(predictor - up),
                    abs(predictor - upper_left),
                )
                row[index] = (
                    value + (left, up, upper_left)[distances.index(min(distances))]
                ) & 255
            else:
                assert filter_type == 0, f"unsupported PNG filter: {filter_type}"
        rows.append(row)
    return data, width, height, rows


first_data, width, height, rows = decode_rgba(sys.argv[1])
second_data, second_width, second_height, _ = decode_rgba(sys.argv[2])
assert (width, height) == (960, 720), "unexpected Creative Art image size"
assert (second_width, second_height) == (width, height), "export sizes differ"
assert first_data == second_data, "fixed seed and time must export identical PNG bytes"

pixels = [bytes(row[index : index + 4]) for row in rows for index in range(0, len(row), 4)]
assert all(pixel[3] == 255 for pixel in pixels), "export must be fully opaque"
assert len(set(pixels)) > 1000, "Creative Art output lacks expected visual variety"
assert rows[10][10 * 4 : 10 * 4 + 4] != rows[360][480 * 4 : 480 * 4 + 4], (
    "art center should differ from the outer background"
)

print(
    "creative-art smoke: deterministic 960x720 RGBA PNG",
    hashlib.sha256(first_data).hexdigest(),
)
PY
