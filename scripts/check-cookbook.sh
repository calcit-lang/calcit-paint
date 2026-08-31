#!/usr/bin/env bash
set -euo pipefail

cookbook_png="$(mktemp "${TMPDIR:-/tmp}/calcit-paint-cookbook.XXXXXX")"
trap 'rm -f "$cookbook_png"' EXIT

calcit ./calcit.cirru eval --dep ./ 'ns cookbook.smoke $ :require
  calcit-paint.core :refer $ validate-scene render-to-png!

let
    no-diagnostics $ []
    basic $ {} (:type :rectangle) (:position ([] 2 2)) (:width 8) (:height 4) (:fill-color ([] 0 100 50))
    nested $ {} (:type :clip-rect) (:position ([] 0 0)) (:width 20) (:height 12)
      :children $ []
        {} (:type :touch-area) (:dx 8) (:dy 4)
          :position $ [] 10 6
          :action :cookbook
          :children $ []
            {} (:type :text) (:text |Cookbook)
              :position $ [] 10 6
              :color $ [] 0 0 96
              :size 12
              :align :center
    focusable $ {} (:type :focus-area) (:focus-id |cookbook-editor) (:text-input? true) (:position ([] 10 6)) (:dx 8) (:dy 4)
      :accessibility $ {} (:id |cookbook-editor) (:role :text-input) (:label "|Cookbook editor") (:value |Draft) (:focusable? true)
    asset $ {} (:type :image) (:file-path |resources/calcit.png) (:x 0) (:y 0) (:w 12) (:h 8) (:fit :contain) (:sampling :linear)
  do
    assert= no-diagnostics $ validate-scene basic
    assert= no-diagnostics $ validate-scene nested
    assert= no-diagnostics $ validate-scene focusable
    assert= no-diagnostics $ validate-scene asset
    render-to-png! $ {} (:path |'"$cookbook_png"') (:width 12) (:height 8) (:scene basic)
    , &unit'

python3 - "$cookbook_png" <<'PY'
import struct
import sys
import zlib

path = sys.argv[1]
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
        width, height, depth, color, compression, filter_method, interlace = struct.unpack(">IIBBBBB", chunk)
        assert (depth, color, compression, filter_method, interlace) == (8, 6, 0, 0, 0), "expected RGBA8888 PNG"
    elif kind == b"IDAT":
        compressed.extend(chunk)

assert (width, height) == (12, 8), "unexpected cookbook image size"
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
            distances = (abs(predictor - left), abs(predictor - up), abs(predictor - upper_left))
            row[index] = (value + (left, up, upper_left)[distances.index(min(distances))]) & 255
        else:
            assert filter_type == 0, f"unsupported PNG filter: {filter_type}"
    rows.append(row)

assert bytes(rows[0][0:4]) == b"\x00\x00\x00\x00", "background pixel changed"
assert bytes(rows[3][3 * 4 : 3 * 4 + 4]) == b"\xff\x00\x00\xff", "rectangle pixel changed"
PY
