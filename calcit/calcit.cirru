
{}
  :users $ {}
    |u0 $ {} (:name |chen) (:id |u0) (:nickname |chen) (:avatar nil) (:password |d41d8cd98f00b204e9800998ecf8427e) (:theme :star-trail)
  :ir $ {} (:package |app)
    :files $ {}
      |app.main $ {}
        :ns $ {} (:type :expr) (:by |u0) (:at 1620576367501)
          :data $ {}
            |T $ {} (:type :leaf) (:by |u0) (:at 1620576367501) (:text |ns)
            |j $ {} (:type :leaf) (:by |u0) (:at 1620576367501) (:text |app.main)
        :defs $ {}
          |main! $ {} (:type :expr) (:by |u0) (:at 1620576371638)
            :data $ {}
              |T $ {} (:type :leaf) (:by |u0) (:at 1620576371638) (:text |defn)
              |j $ {} (:type :leaf) (:by |u0) (:at 1620576371638) (:text |main!)
              |r $ {} (:type :expr) (:by |u0) (:at 1620576371638)
                :data $ {}
              |v $ {} (:type :expr) (:by |u0) (:at 1620576431419)
                :data $ {}
                  |T $ {} (:type :leaf) (:by |u0) (:at 1620576433107) (:text |println)
                  |j $ {} (:type :leaf) (:by |u0) (:at 1620576437037) (:text "|\"started")
          |reload! $ {} (:type :expr) (:by |u0) (:at 1620576374114)
            :data $ {}
              |T $ {} (:type :leaf) (:by |u0) (:at 1620576374114) (:text |defn)
              |j $ {} (:type :leaf) (:by |u0) (:at 1620576374114) (:text |reload!)
              |r $ {} (:type :expr) (:by |u0) (:at 1620576374114)
                :data $ {}
              |v $ {} (:type :expr) (:by |u0) (:at 1620576440021)
                :data $ {}
                  |T $ {} (:type :leaf) (:by |u0) (:at 1620576440821) (:text |println)
                  |j $ {} (:type :leaf) (:by |u0) (:at 1620618549063) (:text "|\"reloads 19")
        :proc $ {} (:type :expr) (:by |u0) (:at 1620576367501)
          :data $ {}
        :configs $ {}
  :configs $ {} (:reload-fn |app.main/reload!)
    :modules $ []
    :output |src
    :port 6001
    :extension |.cljs
    :local-ui? false
    :init-fn |app.main/main!
    :compact-output? true
    :version |0.0.1
