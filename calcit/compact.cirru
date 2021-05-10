
{} (:package |app)
  :configs $ {} (:init-fn |app.main/main!) (:reload-fn |app.main/reload!)
    :modules $ []
    :version |0.0.1
  :files $ {}
    |app.main $ {}
      :ns $ quote (ns app.main)
      :defs $ {}
        |main! $ quote
          defn main! () $ println "\"started"
        |reload! $ quote
          defn reload! () $ println "\"reloads 19"
      :proc $ quote ()
      :configs $ {}
