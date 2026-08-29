
{} (:about "|Machine-generated snapshot. Do not edit directly — changes will be overwritten. Use `calcit query` to inspect and `calcit edit`/`calcit tree` to modify. Run `calcit docs agents --full` first. Manual edits must follow format and schema conventions, then run `calcit edit format`.") (:package |calcit-paint)
  :entries $ {}
    :default $ {} (:description |) (:init-fn 'calcit-paint.main/main!) (:mode :native) (:reload-fn 'calcit-paint.main/reload!)
      :feature-policy $ {}
      :modules $ []
      :type-slots $ {}
  :files $ {}
    |calcit-paint.core $ %{} 'FileEntry
      :defs $ {}
        |blur! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn blur! ()
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |clear_focus
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |focus! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn focus! (id)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |request_focus id
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'String
        |focused? $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn focused? (id)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |focused id
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Bool)
              :args $ [] 'String
        |launch-canvas! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn launch-canvas! (cb)
              &blocking-dylib-edn-fn (get-dylib-path |/dylibs/libcalcit_paint) |launch_canvas $ fn (event) (cb event) :handled
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
                :: 'Fn $ {} (:return 'R)
                  :args $ [] 'Dynamic
              :generics $ [] 'R
        |measure-paragraph! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn measure-paragraph! (data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |measure_paragraph data
          :examples $ []
          :schema $ :: 'Fn
            {}
              :args $ [] (:: 'Map 'Tag 'Dynamic)
              :return $ :: 'Map 'Tag 'Number
        |measure-text! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn measure-text! (data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |measure_text data
          :examples $ []
          :schema $ :: 'Fn
            {}
              :args $ [] (:: 'Map 'Tag 'Dynamic)
              :return $ :: 'Map 'Tag 'Number
        |push-drawing-data! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn push-drawing-data! (op data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |push_drawing_data op data
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'String 'T
              :generics $ [] 'T
        |render-to-png! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn render-to-png! (options)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |render_to_png options
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'T
              :generics $ [] 'T
        |request-frame! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn request-frame! ()
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |request_frame
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |validate-scene $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn validate-scene (scene)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |validate_scene scene
          :examples $ []
          :schema $ :: 'Fn
            {}
              :args $ [] 'T
              :generics $ [] 'T
              :return $ :: 'List 'String
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.core $ :require
            calcit-paint.$meta :refer $ calcit-dirname
            calcit-paint.util :refer $ get-dylib-path
    |calcit-paint.main $ %{} 'FileEntry
      :defs $ {}
        |*animation-active? $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *animation-active? false)
          :examples $ []
          :schema $ :: 'Ref 'Bool
        |*animation-time-ms $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *animation-time-ms 0)
          :examples $ []
          :schema $ :: 'Ref 'Number
        |export-offscreen-demo! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn export-offscreen-demo! ()
              render-to-png! $ {} (:path |offscreen-demo.png) (:width 360) (:height 180)
                :background $ [] 225 25 12
                :scene $ {} (:type :group)
                  :children $ []
                    {} (:type :cached-group) (:cache-key |offscreen-card) (:revision 1)
                      :position $ [] 20 20
                      :width 320
                      :height 140
                      :children $ []
                        {} (:type :rounded-rect)
                          :position $ [] 0 0
                          :width 320
                          :height 140
                          :radius 18
                          :fill-color $ [] 205 70 42
                        {} (:type :circle)
                          :position $ [] 75 70
                          :radius 42
                          :fill-color $ [] 38 90 58
                        {} (:type :text) (:text "|Offscreen · 离屏快照")
                          :position $ [] 140 70
                          :color $ [] 0 0 98
                          :size 22
                          :baseline :middle
                          :align :left
              println "|Exported offscreen-demo.png / 已导出离屏快照"
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |main! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn main! () (println |started)
              println $ measure-text!
                {} (:text "|Text layout / 文本排版") (:size 24) (:font-family |monospace) (:weight 700) (:style :italic) (:baseline :middle)
              println $ measure-paragraph!
                {} (:text "|Paragraph measurement / 段落测量") (:max-width 260) (:size 20) (:line-height 28) (:max-lines 2) (:ellipsis "|…")
              validate-scene-demo!
              render! true
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |reload! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn reload! () (render! false) (println "|reloads 19")
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |render! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn render! (start-loop?)
              push-drawing-data! |reset-canvas! $ [] 200 50 30
              push-drawing-data! |render-canvas! $ {} (:type :group)
                :children $ []
                  {} (:type :rectangle)
                    :position $ [] 80 100
                    :width 100
                    :height 40
                    :fill-color $ [] 200 80 80
                  {} (:type :circle)
                    :position $ [] 120 300
                    :radius 40
                    :fill-color $ [] 0 80 70
                  {} (:type :clip-rect)
                    :position $ [] 220 80
                    :width 260
                    :height 160
                    :children $ []
                      {} (:type :opacity) (:alpha 0.85)
                        :children $ []
                          {} (:type :rounded-rect)
                            :position $ [] 230 90
                            :width 120
                            :height 70
                            :radius 16
                            :fill-color $ [] 45 80 60
                          {} (:type :ellipse)
                            :position $ [] 400 125
                            :radius-x 48
                            :radius-y 30
                            :line-color $ [] 200 80 80
                            :line-width 4
                          {} (:type :arc)
                            :position $ [] 350 205
                            :radius-x 70
                            :radius-y 32
                            :start-angle 190
                            :sweep-angle 160
                            :line-color $ [] 120 80 70
                            :line-width 5
                  {} (:type :text) (:text |Demo)
                    :position $ [] 140 40
                    :color $ [] 0 80 100
                    :size 40
                    :weight |300
                    :align :center
                  {} (:type :group)
                    :children $ []
                      {} (:type :text) (:text "|Bold italic · top")
                        :position $ [] 530 110
                        :color $ [] 42 90 92
                        :size 24
                        :font-family |monospace
                        :weight 700
                        :style :italic
                        :baseline :top
                        :align :left
                      {} (:type :text) (:text "|Regular · middle")
                        :position $ [] 530 158
                        :color $ [] 170 76 96
                        :size 24
                        :font-family |monospace
                        :weight 400
                        :baseline :middle
                        :align :left
                      {} (:type :text) (:text "|Light · bottom")
                        :position $ [] 530 206
                        :color $ [] 42 90 88
                        :size 24
                        :font-family |monospace
                        :weight 300
                        :baseline :bottom
                        :align :left
                  {} (:type :polyline)
                    :position $ [] 480 200
                    :color $ [] 0 0 100 1
                    :skip-first? true
                    :width 2
                    :stops $ -> (range 20)
                      map $ fn (i)
                        []
                          * 80 $ cos (* 1.9 i)
                          * 80 $ sin (* 1.9 i)
                    :join :round
                    :cap :round
                  {} (:type :touch-area) (:radius 10)
                    :position $ [] 200 200
                    :fill-color $ [] 40 80 80
                  {} (:type :key-listener) (:key |D) (:action :keyboard)
                    :path $ [] :k
                    :data :data
                  {} (:type :ops)
                    :path $ []
                      [] :move-to $ [] 200 300
                      [] :line-to $ [] 240 340
                      [] :bezier3-to ([] 400 260) ([] 200 400) ([] 300 400)
                    :line-color $ [] 200 80 80
                    :line-width 4
                  {} (:type :translate) (:x 200) (:y 200)
                    :children $ []
                      {} (:type :scale) (:factor 2)
                        :children $ []
                          {} (:type :rotate) (:radius 0.8)
                            :children $ []
                              {} (:type :rectangle)
                                :position $ [] 0 0
                                :width 100
                                :height 40
                                :fill-color $ [] 200 80 80
                  {} (:type :scale) (:factor 2.5)
                    :children $ []
                      {} (:type :touch-area) (:radius 10)
                        :position $ [] 200 200
                        :fill-color $ [] 40 80 80
                  {} (:type :group)
                    :children $ []
                      {} (:type :rounded-rect)
                        :position $ [] 500 410
                        :width 180
                        :height 64
                        :radius 14
                        :fill $ {} (:type :linear-gradient)
                          :from $ [] 500 410
                          :to $ [] 680 474
                          :stops $ []
                            [] 0 $ [] 16 90 60
                            [] 0.5 $ [] 330 85 62
                            [] 1 $ [] 210 85 55
                      {} (:type :circle)
                        :position $ [] 790 442
                        :radius 52
                        :fill $ {} (:type :radial-gradient)
                          :center $ [] 772 424
                          :radius 70
                          :stops $ []
                            [] 0 $ [] 52 95 72
                            [] 0.55 $ [] 18 90 58
                            [] 1 $ [] 348 85 42
                      {} (:type :rectangle)
                        :position $ [] 500 510
                        :width 180
                        :height 58
                        :stroke $ {}
                          :paint $ {} (:type :solid)
                            :color $ [] 192 90 68
                          :width 5
                          :cap :round
                          :join :miter
                          :miter-limit 6
                          :dash $ [] 14 8
                          :dash-offset 3
                      {} (:type :rectangle)
                        :position $ [] 750 510
                        :width 140
                        :height 70
                        :fill-color $ [] 48 90 62
                      {} (:type :blend) (:mode :multiply)
                        :children $ []
                          {} (:type :circle)
                            :position $ [] 830 545
                            :radius 42
                            :fill-color $ [] 215 90 60
                  {} (:type :group)
                    :children $ []
                      {} (:type :touch-area) (:dx 150) (:dy 40)
                        :position $ [] 780 320
                        :action :input-demo
                        :path $ [] :demo :pointer
                        :data :pointer-demo
                        :fill-color $ [] 185 70 50
                        :line-color $ [] 185 90 85
                        :line-width 2
                      {} (:type :text) (:text "|Pointer event demo")
                        :position $ [] 780 314
                        :color $ [] 0 0 100
                        :size 18
                        :align :center
                      {} (:type :text) (:text "|Click or drag; hold Shift; press I")
                        :position $ [] 780 338
                        :color $ [] 0 0 94
                        :size 13
                        :align :center
                      {} (:type :key-listener) (:key |I) (:action :input-demo)
                        :path $ [] :demo :keyboard
                        :data :keyboard-demo
                  {} (:type :group)
                    :children $ []
                      {} (:type :paragraph) (:text "|Calcit Paint paragraph\n中文段落 · explicit newline")
                        :position $ [] 40 610
                        :max-width 300
                        :color $ [] 42 90 92
                        :size 20
                        :line-height 28
                        :align :left
                      {} (:type :paragraph) (:text "|A constrained paragraph demonstrates Unicode-safe wrapping and ellipsis. 受限宽度段落展示安全换行与省略号。")
                        :position $ [] 380 610
                        :max-width 320
                        :color $ [] 170 76 96
                        :size 18
                        :line-height 26
                        :max-lines 2
                        :ellipsis "|…"
                        :align :center
                      {} (:type :paragraph) (:text "|مرحبا بالعالم · تخطيط النص من اليمين إلى اليسار")
                        :position $ [] 740 610
                        :max-width 320
                        :color $ [] 200 82 90
                        :size 20
                        :line-height 30
                        :max-lines 2
                        :ellipsis "|…"
                        :direction :rtl
                        :align :right
                  {} (:type :group)
                    :children $ []
                      {} (:type :focus-area) (:focus-id |field-a) (:tab-index 0) (:text-input? true)
                        :position $ [] 180 450
                        :dx 140
                        :dy 32
                        :action :focus-demo
                        :path $ [] :field-a
                        :data :ime-field
                        :fill-color $ [] 215 70 45
                        :line-color $ [] 215 88 76
                        :line-width 3
                      {} (:type :text) (:text "|Focus A · IME text input")
                        :position $ [] 180 450
                        :color $ [] 0 0 98
                        :size 18
                        :baseline :middle
                        :align :center
                      {} (:type :focus-area) (:focus-id |field-b) (:tab-index 1) (:text-input? true)
                        :position $ [] 180 525
                        :dx 140
                        :dy 32
                        :action :focus-demo
                        :path $ [] :field-b
                        :data :ime-field
                        :fill-color $ [] 165 65 42
                        :line-color $ [] 165 88 74
                        :line-width 3
                      {} (:type :text) (:text "|Focus B · Tab / Shift+Tab")
                        :position $ [] 180 525
                        :color $ [] 0 0 98
                        :size 18
                        :baseline :middle
                        :align :center
                      {} (:type :key-listener) (:key |K) (:action :focus-first)
                        :modifiers $ {} (:shift? true)
                        :data :shortcut-demo
                      {} (:type :key-listener) (:key |P) (:action :export-snapshot)
                        :modifiers $ {} (:shift? true)
                        :data :offscreen-demo
                      {} (:type :key-listener) (:key |Enter) (:focus-id |field-a) (:action :field-submit) (:data :focus-scoped-key)
                      {} (:type :text) (:text "|Click, Tab, IME; Shift+K focuses A; Shift+P exports PNG")
                        :position $ [] 180 580
                        :color $ [] 45 18 96
                        :size 14
                        :align :center
                  {} (:type :image) (:file-path |resources/calcit.png) (:x 400) (:y 40) (:w 80) (:h 80)
                    ; :crop $ {} (:x 0) (:y 0) (:w 200) (:h 200)
                  {} (:type :group)
                    :children $ []
                      {} (:type :circle) (:radius 22)
                        :position $ []
                          + 900 $ * 55
                            sin $ / @*animation-time-ms 420
                          , 270
                        :fill-color $ [] 285 82 62
                        :line-color $ [] 285 95 86
                        :line-width 3
                      {} (:type :text)
                        :text $ if @*animation-active? "|A: pause animation / 暂停动画" "|A: start animation / 开始动画"
                        :position $ [] 900 315
                        :color $ [] 285 72 92
                        :size 15
                        :align :center
                      {} (:type :key-listener) (:key |A) (:action :toggle-animation)
                  {} (:type :cached-group) (:cache-key |window-static-badge) (:revision 1)
                    :position $ [] 900 80
                    :width 170
                    :height 110
                    :children $ []
                      {} (:type :rounded-rect)
                        :position $ [] 0 0
                        :width 170
                        :height 110
                        :radius 16
                        :fill-color $ [] 225 55 28
                        :line-color $ [] 195 82 68
                        :line-width 3
                      {} (:type :circle)
                        :position $ [] 42 55
                        :radius 25
                        :fill-color $ [] 42 90 58
                      {} (:type :text) (:text |cached)
                        :position $ [] 80 48
                        :color $ [] 0 0 96
                        :size 18
                        :align :left
                      {} (:type :text) (:text "|Shift+P → PNG")
                        :position $ [] 80 72
                        :color $ [] 0 0 86
                        :size 13
                        :align :left
              if start-loop? $ launch-canvas!
                fn (event)
                  if (map? event)
                    case-default
                      .unwrap-or (get event :action) :none
                      case-default
                        .unwrap-or (get event :type) :unknown
                        println |event: event
                        :redraw $ render! false
                        :frame $ do
                          reset! *animation-time-ms $ .unwrap-or (get event :timestamp-ms) @*animation-time-ms
                          if @*animation-active? $ do (render! false) (request-frame!)
                      :focus-first $ focus! |field-a
                      :export-snapshot $ export-offscreen-demo!
                      :toggle-animation $ toggle-animation!
                    do (println |event: event) (request-frame!)
                  , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'Bool
        |toggle-animation! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn toggle-animation! ()
              if @*animation-active?
                do (reset! *animation-active? false) (render! false)
                do (reset! *animation-active? true) (request-frame!)
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |validate-scene-demo! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn validate-scene-demo! () $ let
                valid-scene $ {} (:type :group)
                  :children $ []
                    {} (:type :rounded-rect) (:width 160) (:height 70) (:radius 12)
                valid-diagnostics $ validate-scene valid-scene
                invalid-diagnostics $ validate-scene
                  {} (:type :group)
                    :children $ []
                      {} $ :type :unknown-demo-shape
                      {} (:type :group)
                        :children $ [] true
              if (empty? valid-diagnostics) (println "|scene validation passed / 场景校验通过")
                raise $ str "|unexpected diagnostics for valid scene: " valid-diagnostics
              if
                = 2 $ count invalid-diagnostics
                println "|expected validation diagnostics / 预期校验诊断: " invalid-diagnostics
                raise $ str "|expected two invalid-scene diagnostics, got: " invalid-diagnostics
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.main $ :require
            calcit-paint.core :refer $ launch-canvas! push-drawing-data! measure-text! measure-paragraph! focus! render-to-png! validate-scene request-frame!
    |calcit-paint.util $ %{} 'FileEntry
      :defs $ {}
        |get-dylib-ext $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defmacro get-dylib-ext () $ case-default (&get-os) |.so (:macos |.dylib) (:windows |.dll)
          :examples $ []
          :schema $ :: 'Macro
            {}
              :capabilities $ #{} :platform-read
              :expansion $ :: 'Expr 'String
              :required $ []
        |get-dylib-path $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn get-dylib-path (p)
              str (or-current-path calcit-dirname) p $ get-dylib-ext
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'String)
              :args $ [] 'String
        |or-current-path $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn or-current-path (p)
              if (blank? p) |. p
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'String)
              :args $ [] 'String
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.util $ :require
            calcit-paint.$meta :refer $ calcit-dirname calcit-filename
