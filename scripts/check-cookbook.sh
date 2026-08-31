#!/usr/bin/env bash
set -euo pipefail

calcit ./calcit.cirru eval --dep ./ 'ns cookbook.smoke $ :require
  calcit-paint.core :refer $ validate-scene render-to-png!

let
    no-diagnostics $ []
    basic $ {} (:type :rectangle) (:position ([] 2 2)) (:width 8) (:height 4) (:fill-color ([] 210 72 48))
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
    render-to-png! $ {} (:path |cookbook-smoke.png) (:width 12) (:height 8) (:scene basic)
    , &unit'

test "$(od -An -tx1 -N8 cookbook-smoke.png | tr -d ' \\n')" = "89504e470d0a1a0a"
rm cookbook-smoke.png
