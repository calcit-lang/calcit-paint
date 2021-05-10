
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
        |reload! $ quote
          defn reload! () (render!) (println "\"reloads 19")
        |render! $ quote
          defn render! ()
            &ffi-message "\"reset-canvas!" $ [] 200 50 70
            &ffi-message "\"render-canvas!" $ {} (:kind :group)
              :children $ []
                {} (:kind :rectangle)
                  :position $ [] 80 100
                  :width 100
                  :height 40
                  :fill-color $ [] 0 80 70
                {} (:kind :circle)
                  :position $ [] 120 300
                  :radius 40
                  :fill-color $ [] 0 80 70
      :proc $ quote ()
      :configs $ {}
