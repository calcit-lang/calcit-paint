
{}
  :configs $ {} (:compact-output? true) (:extension |.cljs) (:init-fn |calcit-paint.main/main!) (:local-ui? false) (:output |src) (:port 6001) (:reload-fn |calcit-paint.main/reload!) (:version |0.0.6)
    :modules $ []
  :entries $ {}
  :ir $ {} (:package |calcit-paint)
    :files $ {}
      |calcit-paint.core $ {}
        :configs $ {}
        :defs $ {}
          |launch-canvas! $ {} (:at 1635409806386) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1635409806386) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1635409841841) (:by |u0) (:text |launch-canvas!) (:type :leaf)
              |n $ {} (:at 1635409858795) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635410509431) (:by |u0) (:text |cb) (:type :leaf)
              |r $ {} (:at 1635409851113) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635414492245) (:by |u0) (:text |&blocking-dylib-edn-fn) (:type :leaf)
                  |j $ {} (:at 1635409851113) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409851113) (:by |u0) (:text |get-dylib-path) (:type :leaf)
                      |j $ {} (:at 1635410504089) (:by |u0) (:text "|\"/dylibs/libcalcit_paint") (:type :leaf)
                  |r $ {} (:at 1635409873536) (:by |u0) (:text "|\"launch_canvas") (:type :leaf)
                  |v $ {} (:at 1635410513138) (:by |u0) (:text |cb) (:type :leaf)
          |push-drawing-data! $ {} (:at 1635409825540) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1635409825540) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1635409825540) (:by |u0) (:text |push-drawing-data!) (:type :leaf)
              |n $ {} (:at 1635409907590) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409912017) (:by |u0) (:text |op) (:type :leaf)
                  |j $ {} (:at 1635409912592) (:by |u0) (:text |data) (:type :leaf)
              |r $ {} (:at 1635409905980) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409905980) (:by |u0) (:text |&call-dylib-edn) (:type :leaf)
                  |j $ {} (:at 1635409905980) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409905980) (:by |u0) (:text |get-dylib-path) (:type :leaf)
                      |j $ {} (:at 1635410518336) (:by |u0) (:text "|\"/dylibs/libcalcit_paint") (:type :leaf)
                  |r $ {} (:at 1635409926856) (:by |u0) (:text "|\"push_drawing_data") (:type :leaf)
                  |v $ {} (:at 1635409928112) (:by |u0) (:text |op) (:type :leaf)
                  |x $ {} (:at 1635409928662) (:by |u0) (:text |data) (:type :leaf)
        :ns $ {} (:at 1635409672442) (:by |u0) (:type :expr)
          :data $ {}
            |T $ {} (:at 1635409672442) (:by |u0) (:text |ns) (:type :leaf)
            |j $ {} (:at 1635409672442) (:by |u0) (:text |calcit-paint.core) (:type :leaf)
            |r $ {} (:at 1635409786282) (:by |u0) (:type :expr)
              :data $ {}
                |T $ {} (:at 1635409786282) (:by |u0) (:text |:require) (:type :leaf)
                |j $ {} (:at 1635409786282) (:by |u0) (:type :expr)
                  :data $ {}
                    |T $ {} (:at 1635410464249) (:by |u0) (:text |calcit-paint.$meta) (:type :leaf)
                    |j $ {} (:at 1635409786282) (:by |u0) (:text |:refer) (:type :leaf)
                    |r $ {} (:at 1635409786282) (:by |u0) (:type :expr)
                      :data $ {}
                        |T $ {} (:at 1635409786282) (:by |u0) (:text |calcit-dirname) (:type :leaf)
                |r $ {} (:at 1635409786282) (:by |u0) (:type :expr)
                  :data $ {}
                    |T $ {} (:at 1635410468734) (:by |u0) (:text |calcit-paint.util) (:type :leaf)
                    |j $ {} (:at 1635409786282) (:by |u0) (:text |:refer) (:type :leaf)
                    |r $ {} (:at 1635409786282) (:by |u0) (:type :expr)
                      :data $ {}
                        |T $ {} (:at 1635409786282) (:by |u0) (:text |get-dylib-path) (:type :leaf)
        :proc $ {} (:at 1635409672442) (:by |u0) (:type :expr)
          :data $ {}
      |calcit-paint.main $ {}
        :configs $ {}
        :defs $ {}
          |main! $ {} (:at 1620576371638) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1620576371638) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1620576371638) (:by |u0) (:text |main!) (:type :leaf)
              |r $ {} (:at 1620576371638) (:by |u0) (:type :expr)
                :data $ {}
              |v $ {} (:at 1620576431419) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1620576433107) (:by |u0) (:text |println) (:type :leaf)
                  |j $ {} (:at 1620576437037) (:by |u0) (:text "|\"started") (:type :leaf)
              |x $ {} (:at 1620627701008) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1620627701954) (:by |u0) (:text |render!) (:type :leaf)
          |reload! $ {} (:at 1620576374114) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1620576374114) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1620576374114) (:by |u0) (:text |reload!) (:type :leaf)
              |r $ {} (:at 1620576374114) (:by |u0) (:type :expr)
                :data $ {}
              |t $ {} (:at 1620627691943) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1620627695034) (:by |u0) (:text |render!) (:type :leaf)
              |v $ {} (:at 1620576440021) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1620576440821) (:by |u0) (:text |println) (:type :leaf)
                  |j $ {} (:at 1620623062806) (:by |u0) (:text "|\"reloads 19") (:type :leaf)
          |render! $ {} (:at 1620627695917) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1620627695917) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1620627695917) (:by |u0) (:text |render!) (:type :leaf)
              |r $ {} (:at 1620627695917) (:by |u0) (:type :expr)
                :data $ {}
              |t $ {} (:at 1620634596768) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635410412836) (:by |u0) (:text |push-drawing-data!) (:type :leaf)
                  |j $ {} (:at 1620634609565) (:by |u0) (:text "|\"reset-canvas!") (:type :leaf)
                  |r $ {} (:at 1620634610697) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1620634611224) (:by |u0) (:text |[]) (:type :leaf)
                      |j $ {} (:at 1620640299215) (:by |u0) (:text |200) (:type :leaf)
                      |r $ {} (:at 1620640294561) (:by |u0) (:text |50) (:type :leaf)
                      |v $ {} (:at 1620750630728) (:by |u0) (:text |30) (:type :leaf)
              |v $ {} (:at 1620627697351) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635410421870) (:by |u0) (:text |push-drawing-data!) (:type :leaf)
                  |j $ {} (:at 1620627707657) (:by |u0) (:text "|\"render-canvas!") (:type :leaf)
                  |r $ {} (:at 1620646403375) (:by |u0) (:type :expr)
                    :data $ {}
                      |D $ {} (:at 1620646408026) (:by |u0) (:text |{}) (:type :leaf)
                      |L $ {} (:at 1620646409147) (:by |u0) (:type :expr)
                        :data $ {}
                          |T $ {} (:at 1621179667976) (:by |u0) (:text |:type) (:type :leaf)
                          |j $ {} (:at 1620646413885) (:by |u0) (:text |:group) (:type :leaf)
                      |T $ {} (:at 1620646415182) (:by |u0) (:type :expr)
                        :data $ {}
                          |D $ {} (:at 1620646418333) (:by |u0) (:text |:children) (:type :leaf)
                          |T $ {} (:at 1620646419925) (:by |u0) (:type :expr)
                            :data $ {}
                              |D $ {} (:at 1620646420604) (:by |u0) (:text |[]) (:type :leaf)
                              |T $ {} (:at 1620627710698) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1620627711428) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1620627711597) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179581008) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1620627718867) (:by |u0) (:text |:rectangle) (:type :leaf)
                                  |r $ {} (:at 1620639791805) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620639878089) (:by |u0) (:text |:position) (:type :leaf)
                                      |j $ {} (:at 1620639878398) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620639880221) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1620645742223) (:by |u0) (:text |80) (:type :leaf)
                                          |r $ {} (:at 1621851635366) (:by |u0) (:text |100) (:type :leaf)
                                  |v $ {} (:at 1620639882610) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620639884101) (:by |u0) (:text |:width) (:type :leaf)
                                      |j $ {} (:at 1620640310337) (:by |u0) (:text |100) (:type :leaf)
                                  |x $ {} (:at 1620639887708) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620639889950) (:by |u0) (:text |:height) (:type :leaf)
                                      |j $ {} (:at 1620639890886) (:by |u0) (:text |40) (:type :leaf)
                                  |y $ {} (:at 1620645773055) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620645774831) (:by |u0) (:text |:fill-color) (:type :leaf)
                                      |j $ {} (:at 1620645775064) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620645776531) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1620750615085) (:by |u0) (:text |200) (:type :leaf)
                                          |r $ {} (:at 1620645778697) (:by |u0) (:text |80) (:type :leaf)
                                          |v $ {} (:at 1620750620203) (:by |u0) (:text |80) (:type :leaf)
                              |j $ {} (:at 1620627710698) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1620627711428) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1620627711597) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179583543) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1620646535357) (:by |u0) (:text |:circle) (:type :leaf)
                                  |r $ {} (:at 1620639791805) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620639878089) (:by |u0) (:text |:position) (:type :leaf)
                                      |j $ {} (:at 1620639878398) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620639880221) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1620646621724) (:by |u0) (:text |120) (:type :leaf)
                                          |r $ {} (:at 1620646623178) (:by |u0) (:text |300) (:type :leaf)
                                  |v $ {} (:at 1620639882610) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620646541219) (:by |u0) (:text |:radius) (:type :leaf)
                                      |j $ {} (:at 1620646617536) (:by |u0) (:text |40) (:type :leaf)
                                  |y $ {} (:at 1620645773055) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620645774831) (:by |u0) (:text |:fill-color) (:type :leaf)
                                      |j $ {} (:at 1620645775064) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620645776531) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1620645777947) (:by |u0) (:text |0) (:type :leaf)
                                          |r $ {} (:at 1620645778697) (:by |u0) (:text |80) (:type :leaf)
                                          |v $ {} (:at 1620645779299) (:by |u0) (:text |70) (:type :leaf)
                              |r $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1620746959648) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179595537) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1621179593736) (:by |u0) (:text |:text) (:type :leaf)
                                  |n $ {} (:at 1620746999435) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620747000555) (:by |u0) (:text |:text) (:type :leaf)
                                      |j $ {} (:at 1620748320008) (:by |u0) (:text "|\"Demo") (:type :leaf)
                                  |r $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620746959648) (:by |u0) (:text |:position) (:type :leaf)
                                      |j $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620746959648) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1641034843348) (:by |u0) (:text |140) (:type :leaf)
                                          |r $ {} (:at 1641034835392) (:by |u0) (:text |40) (:type :leaf)
                                  |v $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620746959648) (:by |u0) (:text |:color) (:type :leaf)
                                      |j $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620746959648) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1620748193554) (:by |u0) (:text |0) (:type :leaf)
                                          |r $ {} (:at 1620746959648) (:by |u0) (:text |80) (:type :leaf)
                                          |v $ {} (:at 1620748197964) (:by |u0) (:text |100) (:type :leaf)
                                  |x $ {} (:at 1620746959648) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620746959648) (:by |u0) (:text |:size) (:type :leaf)
                                      |j $ {} (:at 1620748203518) (:by |u0) (:text |40) (:type :leaf)
                                  |y $ {} (:at 1620747023184) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620747024799) (:by |u0) (:text |:weight) (:type :leaf)
                                      |j $ {} (:at 1620747238140) (:by |u0) (:text "|\"300") (:type :leaf)
                                  |yT $ {} (:at 1631032997506) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631032999464) (:by |u0) (:text |:align) (:type :leaf)
                                      |j $ {} (:at 1631033001972) (:by |u0) (:text |:center) (:type :leaf)
                              |v $ {} (:at 1620749095558) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1620749096272) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1620749096596) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179590472) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1620749100422) (:by |u0) (:text |:polyline) (:type :leaf)
                                  |r $ {} (:at 1620749101002) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620749102380) (:by |u0) (:text |:position) (:type :leaf)
                                      |j $ {} (:at 1620749102892) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620749103596) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1641034811987) (:by |u0) (:text |480) (:type :leaf)
                                          |r $ {} (:at 1620749279337) (:by |u0) (:text |200) (:type :leaf)
                                  |v $ {} (:at 1620749108089) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620749110523) (:by |u0) (:text |:color) (:type :leaf)
                                      |j $ {} (:at 1620749110742) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1620749110997) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1620749112026) (:by |u0) (:text |0) (:type :leaf)
                                          |r $ {} (:at 1620749112285) (:by |u0) (:text |0) (:type :leaf)
                                          |v $ {} (:at 1620749112807) (:by |u0) (:text |100) (:type :leaf)
                                          |x $ {} (:at 1620749273737) (:by |u0) (:text |1) (:type :leaf)
                                  |x $ {} (:at 1620749114230) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620749117647) (:by |u0) (:text |:skip-first?) (:type :leaf)
                                      |j $ {} (:at 1620749118430) (:by |u0) (:text |true) (:type :leaf)
                                  |y $ {} (:at 1620749121022) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179691313) (:by |u0) (:text |:width) (:type :leaf)
                                      |j $ {} (:at 1620749127107) (:by |u0) (:text |2) (:type :leaf)
                                  |yT $ {} (:at 1620749127661) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1620749132684) (:by |u0) (:text |:stops) (:type :leaf)
                                      |b $ {} (:at 1620750451560) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |D $ {} (:at 1620750453530) (:by |u0) (:text |->) (:type :leaf)
                                          |T $ {} (:at 1620750446133) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1620750447076) (:by |u0) (:text |range) (:type :leaf)
                                              |j $ {} (:at 1620750508466) (:by |u0) (:text |20) (:type :leaf)
                                          |j $ {} (:at 1620750454123) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1620750454657) (:by |u0) (:text |map) (:type :leaf)
                                              |j $ {} (:at 1620750454994) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1620750455591) (:by |u0) (:text |fn) (:type :leaf)
                                                  |j $ {} (:at 1620750455927) (:by |u0) (:type :expr)
                                                    :data $ {}
                                                      |T $ {} (:at 1620750456449) (:by |u0) (:text |i) (:type :leaf)
                                                  |r $ {} (:at 1620750458282) (:by |u0) (:type :expr)
                                                    :data $ {}
                                                      |T $ {} (:at 1620750458778) (:by |u0) (:text |[]) (:type :leaf)
                                                      |j $ {} (:at 1620750467786) (:by |u0) (:type :expr)
                                                        :data $ {}
                                                          |D $ {} (:at 1620750468524) (:by |u0) (:text |*) (:type :leaf)
                                                          |L $ {} (:at 1620750483109) (:by |u0) (:text |80) (:type :leaf)
                                                          |T $ {} (:at 1620750460481) (:by |u0) (:type :expr)
                                                            :data $ {}
                                                              |T $ {} (:at 1620750461626) (:by |u0) (:text |cos) (:type :leaf)
                                                              |j $ {} (:at 1620750464588) (:by |u0) (:type :expr)
                                                                :data $ {}
                                                                  |D $ {} (:at 1620750465253) (:by |u0) (:text |*) (:type :leaf)
                                                                  |L $ {} (:at 1620750502240) (:by |u0) (:text |1.9) (:type :leaf)
                                                                  |T $ {} (:at 1620750462383) (:by |u0) (:text |i) (:type :leaf)
                                                      |r $ {} (:at 1620750467786) (:by |u0) (:type :expr)
                                                        :data $ {}
                                                          |D $ {} (:at 1620750468524) (:by |u0) (:text |*) (:type :leaf)
                                                          |L $ {} (:at 1620750485813) (:by |u0) (:text |80) (:type :leaf)
                                                          |T $ {} (:at 1620750460481) (:by |u0) (:type :expr)
                                                            :data $ {}
                                                              |T $ {} (:at 1620750473710) (:by |u0) (:text |sin) (:type :leaf)
                                                              |j $ {} (:at 1620750464588) (:by |u0) (:type :expr)
                                                                :data $ {}
                                                                  |D $ {} (:at 1620750465253) (:by |u0) (:text |*) (:type :leaf)
                                                                  |L $ {} (:at 1620750503958) (:by |u0) (:text |1.9) (:type :leaf)
                                                                  |T $ {} (:at 1620750462383) (:by |u0) (:text |i) (:type :leaf)
                                  |yj $ {} (:at 1620749156160) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179677746) (:by |u0) (:text |:join) (:type :leaf)
                                      |j $ {} (:at 1620749173797) (:by |u0) (:text |:round) (:type :leaf)
                                  |yr $ {} (:at 1620749156160) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179679843) (:by |u0) (:text |:cap) (:type :leaf)
                                      |j $ {} (:at 1620749173797) (:by |u0) (:text |:round) (:type :leaf)
                              |x $ {} (:at 1621018607089) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1621018608336) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1621018608877) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179598941) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1621018614481) (:by |u0) (:text |:touch-area) (:type :leaf)
                                  |r $ {} (:at 1621018630122) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621018638257) (:by |u0) (:text |:radius) (:type :leaf)
                                      |j $ {} (:at 1621018750192) (:by |u0) (:text |10) (:type :leaf)
                                  |t $ {} (:at 1621018680593) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621018681876) (:by |u0) (:text |:action) (:type :leaf)
                                      |j $ {} (:at 1621018682479) (:by |u0) (:text |nil) (:type :leaf)
                                  |v $ {} (:at 1621018641655) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621018679268) (:by |u0) (:text |:path) (:type :leaf)
                                      |j $ {} (:at 1621018679934) (:by |u0) (:text |nil) (:type :leaf)
                                  |x $ {} (:at 1621018683368) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621018686471) (:by |u0) (:text |:data) (:type :leaf)
                                      |j $ {} (:at 1621018688803) (:by |u0) (:text |nil) (:type :leaf)
                                  |y $ {} (:at 1621018689580) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621018692730) (:by |u0) (:text |:position) (:type :leaf)
                                      |j $ {} (:at 1621018693470) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1621018694278) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1621018697029) (:by |u0) (:text |200) (:type :leaf)
                                          |r $ {} (:at 1621018698502) (:by |u0) (:text |200) (:type :leaf)
                                  |yT $ {} (:at 1621018699698) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621018727443) (:by |u0) (:text |:fill-color) (:type :leaf)
                                      |j $ {} (:at 1621018728569) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1621018730060) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1631124908378) (:by |u0) (:text |40) (:type :leaf)
                                          |r $ {} (:at 1621018734828) (:by |u0) (:text |80) (:type :leaf)
                                          |v $ {} (:at 1621018735527) (:by |u0) (:text |80) (:type :leaf)
                              |y $ {} (:at 1621096072396) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1621096073407) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1621096073864) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621179601837) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1621096080661) (:by |u0) (:text |:key-listener) (:type :leaf)
                                  |r $ {} (:at 1621096082650) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621096084326) (:by |u0) (:text |:key) (:type :leaf)
                                      |j $ {} (:at 1621096085295) (:by |u0) (:text "|\"D") (:type :leaf)
                                  |v $ {} (:at 1621096088220) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621096091465) (:by |u0) (:text |:action) (:type :leaf)
                                      |j $ {} (:at 1621096095944) (:by |u0) (:text |:keyboard) (:type :leaf)
                                  |x $ {} (:at 1621096098024) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621096099964) (:by |u0) (:text |:path) (:type :leaf)
                                      |j $ {} (:at 1621096101456) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1621096102313) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1621096103571) (:by |u0) (:text |:k) (:type :leaf)
                                  |y $ {} (:at 1621096104591) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621096105726) (:by |u0) (:text |:data) (:type :leaf)
                                      |j $ {} (:at 1621096106919) (:by |u0) (:text |:data) (:type :leaf)
                              |yT $ {} (:at 1621914650517) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1621914651190) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1621914653051) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621914655190) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1621914676779) (:by |u0) (:text |:ops) (:type :leaf)
                                  |r $ {} (:at 1621914677691) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621914679502) (:by |u0) (:text |:path) (:type :leaf)
                                      |j $ {} (:at 1621914679801) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1621914680860) (:by |u0) (:text |[][]) (:type :leaf)
                                          |j $ {} (:at 1621914681127) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1621914682814) (:by |u0) (:text |:move-to) (:type :leaf)
                                              |j $ {} (:at 1621914683557) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1621914683792) (:by |u0) (:text |[]) (:type :leaf)
                                                  |j $ {} (:at 1621914852689) (:by |u0) (:text |200) (:type :leaf)
                                                  |r $ {} (:at 1621914894732) (:by |u0) (:text |300) (:type :leaf)
                                          |r $ {} (:at 1621914686484) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1621914696802) (:by |u0) (:text |:line-to) (:type :leaf)
                                              |j $ {} (:at 1621914697406) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1621914697774) (:by |u0) (:text |[]) (:type :leaf)
                                                  |j $ {} (:at 1621914904932) (:by |u0) (:text |240) (:type :leaf)
                                                  |r $ {} (:at 1641034904407) (:by |u0) (:text |340) (:type :leaf)
                                          |v $ {} (:at 1621914705308) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1621915493849) (:by |u0) (:text |:bezier3-to) (:type :leaf)
                                              |j $ {} (:at 1621914782122) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1621914783939) (:by |u0) (:text |[]) (:type :leaf)
                                                  |j $ {} (:at 1621914790342) (:by |u0) (:text |400) (:type :leaf)
                                                  |r $ {} (:at 1641034892456) (:by |u0) (:text |260) (:type :leaf)
                                              |r $ {} (:at 1621914782122) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1621914783939) (:by |u0) (:text |[]) (:type :leaf)
                                                  |f $ {} (:at 1621914798006) (:by |u0) (:text |200) (:type :leaf)
                                                  |r $ {} (:at 1621914796431) (:by |u0) (:text |400) (:type :leaf)
                                              |v $ {} (:at 1621914799298) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1621914799499) (:by |u0) (:text |[]) (:type :leaf)
                                                  |j $ {} (:at 1621914800633) (:by |u0) (:text |300) (:type :leaf)
                                                  |r $ {} (:at 1621914910507) (:by |u0) (:text |400) (:type :leaf)
                                  |v $ {} (:at 1621914803927) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621914861880) (:by |u0) (:text |:line-color) (:type :leaf)
                                      |j $ {} (:at 1621914806355) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1621914843432) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1621914811172) (:by |u0) (:text |200) (:type :leaf)
                                          |r $ {} (:at 1621914812853) (:by |u0) (:text |80) (:type :leaf)
                                          |v $ {} (:at 1621914813563) (:by |u0) (:text |80) (:type :leaf)
                                  |x $ {} (:at 1621914865980) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1621914868652) (:by |u0) (:text |:line-width) (:type :leaf)
                                      |j $ {} (:at 1621914869387) (:by |u0) (:text |4) (:type :leaf)
                              |yj $ {} (:at 1631124828282) (:by |u0) (:type :expr)
                                :data $ {}
                                  |D $ {} (:at 1631124833560) (:by |u0) (:text |{}) (:type :leaf)
                                  |L $ {} (:at 1631124836728) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631124838938) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1631124840680) (:by |u0) (:text |:translate) (:type :leaf)
                                  |P $ {} (:at 1631124841372) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631124842306) (:by |u0) (:text |:x) (:type :leaf)
                                      |r $ {} (:at 1631125799807) (:by |u0) (:text |200) (:type :leaf)
                                  |R $ {} (:at 1631124847649) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631124848774) (:by |u0) (:text |:y) (:type :leaf)
                                      |j $ {} (:at 1631125801025) (:by |u0) (:text |200) (:type :leaf)
                                  |T $ {} (:at 1631124850351) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |D $ {} (:at 1631124853585) (:by |u0) (:text |:children) (:type :leaf)
                                      |T $ {} (:at 1631124855274) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |D $ {} (:at 1631124855923) (:by |u0) (:text |[]) (:type :leaf)
                                          |L $ {} (:at 1631125040427) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1631125040427) (:by |u0) (:text |{}) (:type :leaf)
                                              |j $ {} (:at 1631125040427) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631125040427) (:by |u0) (:text |:type) (:type :leaf)
                                                  |j $ {} (:at 1631125040427) (:by |u0) (:text |:scale) (:type :leaf)
                                              |r $ {} (:at 1631125040427) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631125040427) (:by |u0) (:text |:factor) (:type :leaf)
                                                  |j $ {} (:at 1631125040427) (:by |u0) (:text |2) (:type :leaf)
                                              |v $ {} (:at 1631125040427) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631125040427) (:by |u0) (:text |:children) (:type :leaf)
                                                  |j $ {} (:at 1631125040427) (:by |u0) (:type :expr)
                                                    :data $ {}
                                                      |T $ {} (:at 1631125040427) (:by |u0) (:text |[]) (:type :leaf)
                                                      |j $ {} (:at 1631125053948) (:by |u0) (:type :expr)
                                                        :data $ {}
                                                          |T $ {} (:at 1631125053948) (:by |u0) (:text |{}) (:type :leaf)
                                                          |j $ {} (:at 1631125053948) (:by |u0) (:type :expr)
                                                            :data $ {}
                                                              |T $ {} (:at 1631125053948) (:by |u0) (:text |:type) (:type :leaf)
                                                              |j $ {} (:at 1631125053948) (:by |u0) (:text |:rotate) (:type :leaf)
                                                          |r $ {} (:at 1631125053948) (:by |u0) (:type :expr)
                                                            :data $ {}
                                                              |T $ {} (:at 1631125053948) (:by |u0) (:text |:radius) (:type :leaf)
                                                              |j $ {} (:at 1631125814778) (:by |u0) (:text |0.8) (:type :leaf)
                                                          |v $ {} (:at 1631125080922) (:by |u0) (:type :expr)
                                                            :data $ {}
                                                              |T $ {} (:at 1631125053948) (:by |u0) (:text |:children) (:type :leaf)
                                                              |j $ {} (:at 1631125084187) (:by |u0) (:type :expr)
                                                                :data $ {}
                                                                  |T $ {} (:at 1631125089546) (:by |u0) (:text |[]) (:type :leaf)
                                                                  |j $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                    :data $ {}
                                                                      |T $ {} (:at 1631125094190) (:by |u0) (:text |{}) (:type :leaf)
                                                                      |j $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                        :data $ {}
                                                                          |T $ {} (:at 1631125094190) (:by |u0) (:text |:type) (:type :leaf)
                                                                          |j $ {} (:at 1631125094190) (:by |u0) (:text |:rectangle) (:type :leaf)
                                                                      |r $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                        :data $ {}
                                                                          |T $ {} (:at 1631125094190) (:by |u0) (:text |:position) (:type :leaf)
                                                                          |j $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                            :data $ {}
                                                                              |T $ {} (:at 1631125094190) (:by |u0) (:text |[]) (:type :leaf)
                                                                              |j $ {} (:at 1631125094190) (:by |u0) (:text |0) (:type :leaf)
                                                                              |r $ {} (:at 1631125094190) (:by |u0) (:text |0) (:type :leaf)
                                                                      |v $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                        :data $ {}
                                                                          |T $ {} (:at 1631125094190) (:by |u0) (:text |:width) (:type :leaf)
                                                                          |j $ {} (:at 1631125094190) (:by |u0) (:text |100) (:type :leaf)
                                                                      |x $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                        :data $ {}
                                                                          |T $ {} (:at 1631125094190) (:by |u0) (:text |:height) (:type :leaf)
                                                                          |j $ {} (:at 1631125094190) (:by |u0) (:text |40) (:type :leaf)
                                                                      |y $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                        :data $ {}
                                                                          |T $ {} (:at 1631125094190) (:by |u0) (:text |:fill-color) (:type :leaf)
                                                                          |j $ {} (:at 1631125094190) (:by |u0) (:type :expr)
                                                                            :data $ {}
                                                                              |T $ {} (:at 1631125094190) (:by |u0) (:text |[]) (:type :leaf)
                                                                              |j $ {} (:at 1631125094190) (:by |u0) (:text |200) (:type :leaf)
                                                                              |r $ {} (:at 1631125094190) (:by |u0) (:text |80) (:type :leaf)
                                                                              |v $ {} (:at 1631125094190) (:by |u0) (:text |80) (:type :leaf)
                              |yr $ {} (:at 1631126586096) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1631126586625) (:by |u0) (:text |{}) (:type :leaf)
                                  |j $ {} (:at 1631126587220) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631126590660) (:by |u0) (:text |:type) (:type :leaf)
                                      |j $ {} (:at 1631126592836) (:by |u0) (:text |:scale) (:type :leaf)
                                  |n $ {} (:at 1631126601482) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631126603973) (:by |u0) (:text |:factor) (:type :leaf)
                                      |j $ {} (:at 1641034796765) (:by |u0) (:text |2.5) (:type :leaf)
                                  |r $ {} (:at 1631126593446) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1631126594816) (:by |u0) (:text |:children) (:type :leaf)
                                      |j $ {} (:at 1631126595129) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1631126595419) (:by |u0) (:text |[]) (:type :leaf)
                                          |j $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1631126598700) (:by |u0) (:text |{}) (:type :leaf)
                                              |j $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:type) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:text |:touch-area) (:type :leaf)
                                              |r $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:radius) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:text |10) (:type :leaf)
                                              |v $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:action) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:text |nil) (:type :leaf)
                                              |x $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:path) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:text |nil) (:type :leaf)
                                              |y $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:data) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:text |nil) (:type :leaf)
                                              |yT $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:position) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                    :data $ {}
                                                      |T $ {} (:at 1631126598700) (:by |u0) (:text |[]) (:type :leaf)
                                                      |j $ {} (:at 1631126598700) (:by |u0) (:text |200) (:type :leaf)
                                                      |r $ {} (:at 1631126598700) (:by |u0) (:text |200) (:type :leaf)
                                              |yj $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                :data $ {}
                                                  |T $ {} (:at 1631126598700) (:by |u0) (:text |:fill-color) (:type :leaf)
                                                  |j $ {} (:at 1631126598700) (:by |u0) (:type :expr)
                                                    :data $ {}
                                                      |T $ {} (:at 1631126598700) (:by |u0) (:text |[]) (:type :leaf)
                                                      |j $ {} (:at 1631126598700) (:by |u0) (:text |40) (:type :leaf)
                                                      |r $ {} (:at 1631126598700) (:by |u0) (:text |80) (:type :leaf)
                                                      |v $ {} (:at 1631126598700) (:by |u0) (:text |80) (:type :leaf)
                              |z $ {} (:at 1653470634619) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1653470635303) (:by |u0) (:text |{}) (:type :leaf)
                                  |b $ {} (:at 1653470635670) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1653470637820) (:by |u0) (:text |:type) (:type :leaf)
                                      |b $ {} (:at 1653470639148) (:by |u0) (:text |:image) (:type :leaf)
                                  |h $ {} (:at 1653470640844) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1653470642623) (:by |u0) (:text |:file-path) (:type :leaf)
                                      |b $ {} (:at 1653470646754) (:by |u0) (:text "|\"resources/calcit.png") (:type :leaf)
                                  |l $ {} (:at 1653470647519) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1653470648595) (:by |u0) (:text |:x) (:type :leaf)
                                      |b $ {} (:at 1653470660779) (:by |u0) (:text |400) (:type :leaf)
                                  |o $ {} (:at 1653470650195) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1653470651951) (:by |u0) (:text |:y) (:type :leaf)
                                      |b $ {} (:at 1653470663078) (:by |u0) (:text |40) (:type :leaf)
                                  |q $ {} (:at 1653470653290) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1653470653845) (:by |u0) (:text |:w) (:type :leaf)
                                      |b $ {} (:at 1653470665968) (:by |u0) (:text |80) (:type :leaf)
                                  |s $ {} (:at 1653470667190) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |T $ {} (:at 1653470668727) (:by |u0) (:text |:h) (:type :leaf)
                                      |b $ {} (:at 1653470669333) (:by |u0) (:text |80) (:type :leaf)
                                  |t $ {} (:at 1653471928197) (:by |u0) (:type :expr)
                                    :data $ {}
                                      |D $ {} (:at 1653471974774) (:by |u0) (:text |;) (:type :leaf)
                                      |T $ {} (:at 1653471932032) (:by |u0) (:text |:crop) (:type :leaf)
                                      |b $ {} (:at 1653471932609) (:by |u0) (:type :expr)
                                        :data $ {}
                                          |T $ {} (:at 1653471933727) (:by |u0) (:text |{}) (:type :leaf)
                                          |b $ {} (:at 1653471933984) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1653471934259) (:by |u0) (:text |:x) (:type :leaf)
                                              |b $ {} (:at 1653471934660) (:by |u0) (:text |0) (:type :leaf)
                                          |h $ {} (:at 1653471935546) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1653471936648) (:by |u0) (:text |:y) (:type :leaf)
                                              |b $ {} (:at 1653471936984) (:by |u0) (:text |0) (:type :leaf)
                                          |l $ {} (:at 1653471938178) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1653471938730) (:by |u0) (:text |:w) (:type :leaf)
                                              |b $ {} (:at 1653471940399) (:by |u0) (:text |200) (:type :leaf)
                                          |o $ {} (:at 1653471941549) (:by |u0) (:type :expr)
                                            :data $ {}
                                              |T $ {} (:at 1653471966988) (:by |u0) (:text |:h) (:type :leaf)
                                              |b $ {} (:at 1653471943676) (:by |u0) (:text |200) (:type :leaf)
              |x $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635414507587) (:by |u0) (:text |launch-canvas!) (:type :leaf)
                  |j $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635414507587) (:by |u0) (:text |fn) (:type :leaf)
                      |j $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                        :data $ {}
                          |T $ {} (:at 1635414507587) (:by |u0) (:text |event) (:type :leaf)
                      |r $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                        :data $ {}
                          |T $ {} (:at 1635414507587) (:by |u0) (:text |case-default) (:type :leaf)
                          |j $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                            :data $ {}
                              |T $ {} (:at 1635414507587) (:by |u0) (:text |:type) (:type :leaf)
                              |j $ {} (:at 1635414507587) (:by |u0) (:text |event) (:type :leaf)
                          |r $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                            :data $ {}
                              |T $ {} (:at 1635414507587) (:by |u0) (:text |println) (:type :leaf)
                              |j $ {} (:at 1635414507587) (:by |u0) (:text "|\"event:") (:type :leaf)
                              |r $ {} (:at 1635414507587) (:by |u0) (:text |event) (:type :leaf)
                          |v $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                            :data $ {}
                              |T $ {} (:at 1635414507587) (:by |u0) (:text |:redraw) (:type :leaf)
                              |j $ {} (:at 1635414507587) (:by |u0) (:type :expr)
                                :data $ {}
                                  |T $ {} (:at 1635414507587) (:by |u0) (:text |render!) (:type :leaf)
        :ns $ {} (:at 1620576367501) (:by |u0) (:type :expr)
          :data $ {}
            |T $ {} (:at 1620576367501) (:by |u0) (:text |ns) (:type :leaf)
            |j $ {} (:at 1620576367501) (:by |u0) (:text |calcit-paint.main) (:type :leaf)
            |r $ {} (:at 1635410383751) (:by |u0) (:type :expr)
              :data $ {}
                |T $ {} (:at 1635410384473) (:by |u0) (:text |:require) (:type :leaf)
                |j $ {} (:at 1635410385148) (:by |u0) (:type :expr)
                  :data $ {}
                    |T $ {} (:at 1635410391141) (:by |u0) (:text |calcit-paint.core) (:type :leaf)
                    |j $ {} (:at 1635410391805) (:by |u0) (:text |:refer) (:type :leaf)
                    |r $ {} (:at 1635410392044) (:by |u0) (:type :expr)
                      :data $ {}
                        |T $ {} (:at 1635410394351) (:by |u0) (:text |launch-canvas!) (:type :leaf)
                        |j $ {} (:at 1635410402117) (:by |u0) (:text |push-drawing-data!) (:type :leaf)
        :proc $ {} (:at 1620576367501) (:by |u0) (:type :expr)
          :data $ {}
      |calcit-paint.util $ {}
        :defs $ {}
          |get-dylib-ext $ {} (:at 1635409770042) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1635409770042) (:by |u0) (:text |defmacro) (:type :leaf)
              |j $ {} (:at 1635409770042) (:by |u0) (:text |get-dylib-ext) (:type :leaf)
              |r $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                :data $ {}
              |v $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409770042) (:by |u0) (:text |case-default) (:type :leaf)
                  |j $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409770042) (:by |u0) (:text |&get-os) (:type :leaf)
                  |r $ {} (:at 1635409770042) (:by |u0) (:text "|\".so") (:type :leaf)
                  |v $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409770042) (:by |u0) (:text |:macos) (:type :leaf)
                      |j $ {} (:at 1635409770042) (:by |u0) (:text "|\".dylib") (:type :leaf)
                  |x $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409770042) (:by |u0) (:text |:windows) (:type :leaf)
                      |j $ {} (:at 1635409770042) (:by |u0) (:text "|\".dll") (:type :leaf)
          |get-dylib-path $ {} (:at 1635409770042) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1635409770042) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1635409770042) (:by |u0) (:text |get-dylib-path) (:type :leaf)
              |r $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409770042) (:by |u0) (:text |p) (:type :leaf)
              |v $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409770042) (:by |u0) (:text |str) (:type :leaf)
                  |j $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409770042) (:by |u0) (:text |or-current-path) (:type :leaf)
                      |j $ {} (:at 1635409770042) (:by |u0) (:text |calcit-dirname) (:type :leaf)
                  |r $ {} (:at 1635409770042) (:by |u0) (:text |p) (:type :leaf)
                  |v $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409770042) (:by |u0) (:text |get-dylib-ext) (:type :leaf)
          |or-current-path $ {} (:at 1635409770042) (:by |u0) (:type :expr)
            :data $ {}
              |T $ {} (:at 1635409770042) (:by |u0) (:text |defn) (:type :leaf)
              |j $ {} (:at 1635409770042) (:by |u0) (:text |or-current-path) (:type :leaf)
              |r $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409770042) (:by |u0) (:text |p) (:type :leaf)
              |v $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                :data $ {}
                  |T $ {} (:at 1635409770042) (:by |u0) (:text |if) (:type :leaf)
                  |j $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                    :data $ {}
                      |T $ {} (:at 1635409770042) (:by |u0) (:text |blank?) (:type :leaf)
                      |j $ {} (:at 1635409770042) (:by |u0) (:text |p) (:type :leaf)
                  |r $ {} (:at 1635409770042) (:by |u0) (:text "|\".") (:type :leaf)
                  |v $ {} (:at 1635409770042) (:by |u0) (:text |p) (:type :leaf)
        :ns $ {} (:at 1635409770042) (:by |u0) (:type :expr)
          :data $ {}
            |T $ {} (:at 1635409770042) (:by |u0) (:text |ns) (:type :leaf)
            |j $ {} (:at 1635409770042) (:by |u0) (:text |calcit-paint.util) (:type :leaf)
            |r $ {} (:at 1635409770042) (:by |u0) (:type :expr)
              :data $ {}
                |T $ {} (:at 1635409770042) (:by |u0) (:text |:require) (:type :leaf)
                |j $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                  :data $ {}
                    |T $ {} (:at 1635409770042) (:by |u0) (:text |calcit-paint.$meta) (:type :leaf)
                    |j $ {} (:at 1635409770042) (:by |u0) (:text |:refer) (:type :leaf)
                    |r $ {} (:at 1635409770042) (:by |u0) (:type :expr)
                      :data $ {}
                        |T $ {} (:at 1635409770042) (:by |u0) (:text |calcit-dirname) (:type :leaf)
                        |j $ {} (:at 1635409770042) (:by |u0) (:text |calcit-filename) (:type :leaf)
  :users $ {}
    |u0 $ {} (:avatar nil) (:id |u0) (:name |chen) (:nickname |chen) (:password |d41d8cd98f00b204e9800998ecf8427e) (:theme :star-trail)
