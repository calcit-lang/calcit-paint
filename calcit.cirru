
{} (:about "|Machine-generated snapshot. Do not edit directly — changes will be overwritten. Use `calcit query` to inspect and `calcit edit`/`calcit tree` to modify. Run `calcit docs agents --full` first. Manual edits must follow format and schema conventions, then run `calcit edit format`.") (:package |calcit-paint)
  :entries $ {}
    :default $ {} (:description |) (:init-fn 'calcit-paint.main/main!) (:mode :native) (:reload-fn 'calcit-paint.main/reload!)
      :feature-policy $ {}
      :modules $ []
      :type-slots $ {}
  :files $ {}
    |calcit-paint.core $ %{} 'FileEntry
      :defs $ {}
        |launch-canvas! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn launch-canvas! (cb)
              &blocking-dylib-edn-fn (get-dylib-path |/dylibs/libcalcit_paint) |launch_canvas cb
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Nil)
              :args $ []
                :: 'Fn $ {} (:return 'Dynamic)
                  :args $ [] 'Dynamic
        |push-drawing-data! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn push-drawing-data! (op data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |push_drawing_data op data
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Nil)
              :args $ [] 'String 'Dynamic
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.core $ :require
            calcit-paint.$meta :refer $ calcit-dirname
            calcit-paint.util :refer $ get-dylib-path
    |calcit-paint.main $ %{} 'FileEntry
      :defs $ {}
        |main! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn main! () (println |started) (render!)
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Nil)
              :args $ []
        |reload! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn reload! () (render!) (println "|reloads 19")
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        |render! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn render! ()
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
                  {} (:type :touch-area) (:radius 10) (:action nil) (:path nil) (:data nil)
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
                      {} (:type :touch-area) (:radius 10) (:action nil) (:path nil) (:data nil)
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
                  {} (:type :image) (:file-path |resources/calcit.png) (:x 400) (:y 40) (:w 80) (:h 80)
                    ; :crop $ {} (:x 0) (:y 0) (:w 200) (:h 200)
              launch-canvas! $ fn (event)
                if (map? event)
                  case-default
                    .unwrap-or (get event :type) :unknown
                    println |event: event
                    :redraw $ render!
                  println |event: event
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Nil)
              :args $ []
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.main $ :require
            calcit-paint.core :refer $ launch-canvas! push-drawing-data!
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
