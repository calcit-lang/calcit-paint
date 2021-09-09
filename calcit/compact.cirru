
{} (:package |app)
  :configs $ {} (:init-fn |app.main/main!) (:reload-fn |app.main/reload!)
    :modules $ []
    :version |0.0.1
  :files $ {}
    |app.main $ {}
      :ns $ quote (ns app.main)
      :defs $ {}
        |main! $ quote
          defn main! () (println "\"started") (render!)
        |on-window-event $ quote
          defn on-window-event (event)
            case-default (:type event) (println "\"event:" event)
              :redraw $ render!
        |reload! $ quote
          defn reload! () (render!) (println "\"reloads 19")
        |render! $ quote
          defn render! ()
            &ffi-message "\"reset-canvas!" $ [] 200 50 30
            &ffi-message "\"render-canvas!" $ {} (:type :group)
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
                  :position $ [] 200 100
                  :color $ [] 0 80 100
                  :size 40
                  :weight "\"300"
                  :align :center
                {} (:type :polyline)
                  :position $ [] 400 200
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
                    :line-to $ [] 240 300
                    :bezier3-to ([] 400 200) ([] 200 400) ([] 300 400)
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
