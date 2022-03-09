
{} (:package |calcit-paint)
  :configs $ {} (:init-fn |calcit-paint.main/main!) (:reload-fn |calcit-paint.main/reload!)
    :modules $ []
    :version |0.0.4
  :entries $ {}
  :files $ {}
    |calcit-paint.core $ {}
      :ns $ quote
        ns calcit-paint.core $ :require
          calcit-paint.$meta :refer $ calcit-dirname
          calcit-paint.util :refer $ get-dylib-path
      :defs $ {}
        |launch-canvas! $ quote
          defn launch-canvas! (cb)
            &blocking-dylib-edn-fn (get-dylib-path "\"/dylibs/libcalcit_paint") "\"launch_canvas" cb
        |push-drawing-data! $ quote
          defn push-drawing-data! (op data)
            &call-dylib-edn (get-dylib-path "\"/dylibs/libcalcit_paint") "\"push_drawing_data" op data
    |calcit-paint.main $ {}
      :ns $ quote
        ns calcit-paint.main $ :require
          calcit-paint.core :refer $ launch-canvas! push-drawing-data!
      :defs $ {}
        |main! $ quote
          defn main! () (println "\"started") (render!)
        |reload! $ quote
          defn reload! () (render!) (println "\"reloads 19")
        |render! $ quote
          defn render! ()
            push-drawing-data! "\"reset-canvas!" $ [] 200 50 30
            push-drawing-data! "\"render-canvas!" $ {} (:type :group)
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
                {} (:type :text) (:text "\"Demo")
                  :position $ [] 140 40
                  :color $ [] 0 80 100
                  :size 40
                  :weight "\"300"
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
                {} (:type :key-listener) (:key "\"D") (:action :keyboard)
                  :path $ [] :k
                  :data :data
                {} (:type :ops)
                  :path $ [][]
                    :move-to $ [] 200 300
                    :line-to $ [] 240 340
                    :bezier3-to ([] 400 260) ([] 200 400) ([] 300 400)
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
            launch-canvas! $ fn (event)
              case-default (:type event) (println "\"event:" event)
                :redraw $ render!
    |calcit-paint.util $ {}
      :ns $ quote
        ns calcit-paint.util $ :require
          calcit-paint.$meta :refer $ calcit-dirname calcit-filename
      :defs $ {}
        |get-dylib-ext $ quote
          defmacro get-dylib-ext () $ case-default (&get-os) "\".so" (:macos "\".dylib") (:windows "\".dll")
        |get-dylib-path $ quote
          defn get-dylib-path (p)
            str (or-current-path calcit-dirname) p $ get-dylib-ext
        |or-current-path $ quote
          defn or-current-path (p)
            if (blank? p) "\"." p
