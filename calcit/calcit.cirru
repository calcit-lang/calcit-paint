
{}
  :configs $ {} (:reload-fn |app.main/reload!) (:port 6001) (:output |src) (:compact-output? true) (:version |0.0.1)
    :modules $ []
    :init-fn |app.main/main!
    :local-ui? false
    :extension |.cljs
  :ir $ {} (:package |app)
    :files $ {}
      |app.main $ {}
        :defs $ {}
          |main! $ {}
            :data $ {}
              |T $ {} (:text |defn) (:type :leaf) (:at 1620576371638) (:by |u0)
              |j $ {} (:text |main!) (:type :leaf) (:at 1620576371638) (:by |u0)
              |r $ {}
                :data $ {}
                :type :expr
                :at 1620576371638
                :by |u0
              |v $ {}
                :data $ {}
                  |T $ {} (:text |println) (:type :leaf) (:at 1620576433107) (:by |u0)
                  |j $ {} (:text "|\"started") (:type :leaf) (:at 1620576437037) (:by |u0)
                :type :expr
                :at 1620576431419
                :by |u0
              |x $ {}
                :data $ {}
                  |T $ {} (:text |render!) (:type :leaf) (:at 1620627701954) (:by |u0)
                :type :expr
                :at 1620627701008
                :by |u0
            :type :expr
            :at 1620576371638
            :by |u0
          |on-window-event $ {}
            :data $ {}
              |T $ {} (:text |defn) (:type :leaf) (:at 1621053094438) (:by |u0)
              |j $ {} (:text |on-window-event) (:type :leaf) (:at 1621053094438) (:by |u0)
              |r $ {}
                :data $ {}
                  |T $ {} (:text |event) (:type :leaf) (:at 1621054474236) (:by |u0)
                :type :expr
                :at 1621053094438
                :by |u0
              |x $ {}
                :data $ {}
                  |T $ {} (:text |case-default) (:type :leaf) (:at 1622822277097) (:by |u0)
                  |j $ {}
                    :data $ {}
                      |T $ {} (:text |:type) (:type :leaf) (:at 1622822279163) (:by |u0)
                      |j $ {} (:text |event) (:type :leaf) (:at 1622822279837) (:by |u0)
                    :type :expr
                    :at 1622822278383
                    :by |u0
                  |n $ {}
                    :data $ {}
                      |T $ {} (:text |println) (:type :leaf) (:at 1622822296628) (:by |u0)
                      |j $ {} (:text "|\"event:") (:type :leaf) (:at 1622822296628) (:by |u0)
                      |r $ {} (:text |event) (:type :leaf) (:at 1622822296628) (:by |u0)
                    :type :expr
                    :at 1622822296628
                    :by |u0
                  |v $ {}
                    :data $ {}
                      |T $ {} (:text |:redraw) (:type :leaf) (:at 1622825215924) (:by |u0)
                      |j $ {}
                        :data $ {}
                          |T $ {} (:text |render!) (:type :leaf) (:at 1622822288433) (:by |u0)
                        :type :expr
                        :at 1622822288433
                        :by |u0
                    :type :expr
                    :at 1622822283898
                    :by |u0
                :type :expr
                :at 1622822270965
                :by |u0
            :type :expr
            :at 1621053094438
            :by |u0
          |reload! $ {}
            :data $ {}
              |T $ {} (:text |defn) (:type :leaf) (:at 1620576374114) (:by |u0)
              |j $ {} (:text |reload!) (:type :leaf) (:at 1620576374114) (:by |u0)
              |r $ {}
                :data $ {}
                :type :expr
                :at 1620576374114
                :by |u0
              |t $ {}
                :data $ {}
                  |T $ {} (:text |render!) (:type :leaf) (:at 1620627695034) (:by |u0)
                :type :expr
                :at 1620627691943
                :by |u0
              |v $ {}
                :data $ {}
                  |T $ {} (:text |println) (:type :leaf) (:at 1620576440821) (:by |u0)
                  |j $ {} (:text "|\"reloads 19") (:type :leaf) (:at 1620623062806) (:by |u0)
                :type :expr
                :at 1620576440021
                :by |u0
            :type :expr
            :at 1620576374114
            :by |u0
          |render! $ {}
            :data $ {}
              |T $ {} (:text |defn) (:type :leaf) (:at 1620627695917) (:by |u0)
              |j $ {} (:text |render!) (:type :leaf) (:at 1620627695917) (:by |u0)
              |r $ {}
                :data $ {}
                :type :expr
                :at 1620627695917
                :by |u0
              |t $ {}
                :data $ {}
                  |T $ {} (:text |&ffi-message) (:type :leaf) (:at 1620634601883) (:by |u0)
                  |j $ {} (:text "|\"reset-canvas!") (:type :leaf) (:at 1620634609565) (:by |u0)
                  |r $ {}
                    :data $ {}
                      |T $ {} (:text |[]) (:type :leaf) (:at 1620634611224) (:by |u0)
                      |j $ {} (:text |200) (:type :leaf) (:at 1620640299215) (:by |u0)
                      |r $ {} (:text |50) (:type :leaf) (:at 1620640294561) (:by |u0)
                      |v $ {} (:text |30) (:type :leaf) (:at 1620750630728) (:by |u0)
                    :type :expr
                    :at 1620634610697
                    :by |u0
                :type :expr
                :at 1620634596768
                :by |u0
              |v $ {}
                :data $ {}
                  |T $ {} (:text |&ffi-message) (:type :leaf) (:at 1620627697351) (:by |u0)
                  |j $ {} (:text "|\"render-canvas!") (:type :leaf) (:at 1620627707657) (:by |u0)
                  |r $ {}
                    :data $ {}
                      |D $ {} (:text |{}) (:type :leaf) (:at 1620646408026) (:by |u0)
                      |L $ {}
                        :data $ {}
                          |T $ {} (:text |:type) (:type :leaf) (:at 1621179667976) (:by |u0)
                          |j $ {} (:text |:group) (:type :leaf) (:at 1620646413885) (:by |u0)
                        :type :expr
                        :at 1620646409147
                        :by |u0
                      |T $ {}
                        :data $ {}
                          |D $ {} (:text |:children) (:type :leaf) (:at 1620646418333) (:by |u0)
                          |T $ {}
                            :data $ {}
                              |yT $ {}
                                :data $ {}
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1621914651190) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621914655190) (:by |u0)
                                      |j $ {} (:text |:ops) (:type :leaf) (:at 1621914676779) (:by |u0)
                                    :type :expr
                                    :at 1621914653051
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:path) (:type :leaf) (:at 1621914679502) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[][]) (:type :leaf) (:at 1621914680860) (:by |u0)
                                          |j $ {}
                                            :data $ {}
                                              |T $ {} (:text |:move-to) (:type :leaf) (:at 1621914682814) (:by |u0)
                                              |j $ {}
                                                :data $ {}
                                                  |T $ {} (:text |[]) (:type :leaf) (:at 1621914683792) (:by |u0)
                                                  |j $ {} (:text |200) (:type :leaf) (:at 1621914852689) (:by |u0)
                                                  |r $ {} (:text |300) (:type :leaf) (:at 1621914894732) (:by |u0)
                                                :type :expr
                                                :at 1621914683557
                                                :by |u0
                                            :type :expr
                                            :at 1621914681127
                                            :by |u0
                                          |r $ {}
                                            :data $ {}
                                              |T $ {} (:text |:line-to) (:type :leaf) (:at 1621914696802) (:by |u0)
                                              |j $ {}
                                                :data $ {}
                                                  |T $ {} (:text |[]) (:type :leaf) (:at 1621914697774) (:by |u0)
                                                  |j $ {} (:text |240) (:type :leaf) (:at 1621914904932) (:by |u0)
                                                  |r $ {} (:text |300) (:type :leaf) (:at 1621914902710) (:by |u0)
                                                :type :expr
                                                :at 1621914697406
                                                :by |u0
                                            :type :expr
                                            :at 1621914686484
                                            :by |u0
                                          |v $ {}
                                            :data $ {}
                                              |T $ {} (:text |:bezier3-to) (:type :leaf) (:at 1621915493849) (:by |u0)
                                              |j $ {}
                                                :data $ {}
                                                  |T $ {} (:text |[]) (:type :leaf) (:at 1621914783939) (:by |u0)
                                                  |j $ {} (:text |400) (:type :leaf) (:at 1621914790342) (:by |u0)
                                                  |r $ {} (:text |200) (:type :leaf) (:at 1621914791691) (:by |u0)
                                                :type :expr
                                                :at 1621914782122
                                                :by |u0
                                              |r $ {}
                                                :data $ {}
                                                  |T $ {} (:text |[]) (:type :leaf) (:at 1621914783939) (:by |u0)
                                                  |f $ {} (:text |200) (:type :leaf) (:at 1621914798006) (:by |u0)
                                                  |r $ {} (:text |400) (:type :leaf) (:at 1621914796431) (:by |u0)
                                                :type :expr
                                                :at 1621914782122
                                                :by |u0
                                              |v $ {}
                                                :data $ {}
                                                  |T $ {} (:text |[]) (:type :leaf) (:at 1621914799499) (:by |u0)
                                                  |j $ {} (:text |300) (:type :leaf) (:at 1621914800633) (:by |u0)
                                                  |r $ {} (:text |400) (:type :leaf) (:at 1621914910507) (:by |u0)
                                                :type :expr
                                                :at 1621914799298
                                                :by |u0
                                            :type :expr
                                            :at 1621914705308
                                            :by |u0
                                        :type :expr
                                        :at 1621914679801
                                        :by |u0
                                    :type :expr
                                    :at 1621914677691
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:line-color) (:type :leaf) (:at 1621914861880) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1621914843432) (:by |u0)
                                          |j $ {} (:text |200) (:type :leaf) (:at 1621914811172) (:by |u0)
                                          |r $ {} (:text |80) (:type :leaf) (:at 1621914812853) (:by |u0)
                                          |v $ {} (:text |80) (:type :leaf) (:at 1621914813563) (:by |u0)
                                        :type :expr
                                        :at 1621914806355
                                        :by |u0
                                    :type :expr
                                    :at 1621914803927
                                    :by |u0
                                  |x $ {}
                                    :data $ {}
                                      |T $ {} (:text |:line-width) (:type :leaf) (:at 1621914868652) (:by |u0)
                                      |j $ {} (:text |4) (:type :leaf) (:at 1621914869387) (:by |u0)
                                    :type :expr
                                    :at 1621914865980
                                    :by |u0
                                :type :expr
                                :at 1621914650517
                                :by |u0
                              |yj $ {}
                                :data $ {}
                                  |D $ {} (:text |{}) (:type :leaf) (:at 1631124833560) (:by |u0)
                                  |L $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1631124838938) (:by |u0)
                                      |j $ {} (:text |:translate) (:type :leaf) (:at 1631124840680) (:by |u0)
                                    :type :expr
                                    :at 1631124836728
                                    :by |u0
                                  |P $ {}
                                    :data $ {}
                                      |T $ {} (:text |:x) (:type :leaf) (:at 1631124842306) (:by |u0)
                                      |r $ {} (:text |200) (:type :leaf) (:at 1631125799807) (:by |u0)
                                    :type :expr
                                    :at 1631124841372
                                    :by |u0
                                  |R $ {}
                                    :data $ {}
                                      |T $ {} (:text |:y) (:type :leaf) (:at 1631124848774) (:by |u0)
                                      |j $ {} (:text |200) (:type :leaf) (:at 1631125801025) (:by |u0)
                                    :type :expr
                                    :at 1631124847649
                                    :by |u0
                                  |T $ {}
                                    :data $ {}
                                      |D $ {} (:text |:children) (:type :leaf) (:at 1631124853585) (:by |u0)
                                      |T $ {}
                                        :data $ {}
                                          |D $ {} (:text |[]) (:type :leaf) (:at 1631124855923) (:by |u0)
                                          |L $ {}
                                            :data $ {}
                                              |T $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |{})
                                              |j $ {} (:type :expr) (:by |u0) (:at 1631125040427)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |:type)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |:scale)
                                              |r $ {} (:type :expr) (:by |u0) (:at 1631125040427)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |:factor)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |2)
                                              |v $ {}
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |:children)
                                                  |j $ {}
                                                    :data $ {}
                                                      |T $ {} (:type :leaf) (:by |u0) (:at 1631125040427) (:text |[])
                                                      |j $ {}
                                                        :data $ {}
                                                          |T $ {} (:type :leaf) (:by |u0) (:at 1631125053948) (:text |{})
                                                          |j $ {} (:type :expr) (:by |u0) (:at 1631125053948)
                                                            :data $ {}
                                                              |T $ {} (:type :leaf) (:by |u0) (:at 1631125053948) (:text |:type)
                                                              |j $ {} (:type :leaf) (:by |u0) (:at 1631125053948) (:text |:rotate)
                                                          |r $ {}
                                                            :data $ {}
                                                              |T $ {} (:type :leaf) (:by |u0) (:at 1631125053948) (:text |:radius)
                                                              |j $ {} (:text |0.8) (:type :leaf) (:at 1631125814778) (:by |u0)
                                                            :type :expr
                                                            :at 1631125053948
                                                            :by |u0
                                                          |v $ {}
                                                            :data $ {}
                                                              |T $ {} (:text |:children) (:type :leaf) (:at 1631125053948) (:by |u0)
                                                              |j $ {}
                                                                :data $ {}
                                                                  |T $ {} (:text |[]) (:type :leaf) (:at 1631125089546) (:by |u0)
                                                                  |j $ {}
                                                                    :data $ {}
                                                                      |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |{})
                                                                      |j $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                        :data $ {}
                                                                          |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |:type)
                                                                          |j $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |:rectangle)
                                                                      |r $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                        :data $ {}
                                                                          |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |:position)
                                                                          |j $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                            :data $ {}
                                                                              |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |[])
                                                                              |j $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |0)
                                                                              |r $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |0)
                                                                      |v $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                        :data $ {}
                                                                          |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |:width)
                                                                          |j $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |100)
                                                                      |x $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                        :data $ {}
                                                                          |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |:height)
                                                                          |j $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |40)
                                                                      |y $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                        :data $ {}
                                                                          |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |:fill-color)
                                                                          |j $ {} (:type :expr) (:by |u0) (:at 1631125094190)
                                                                            :data $ {}
                                                                              |T $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |[])
                                                                              |j $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |200)
                                                                              |r $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |80)
                                                                              |v $ {} (:type :leaf) (:by |u0) (:at 1631125094190) (:text |80)
                                                                    :type :expr
                                                                    :at 1631125094190
                                                                    :by |u0
                                                                :type :expr
                                                                :at 1631125084187
                                                                :by |u0
                                                            :type :expr
                                                            :at 1631125080922
                                                            :by |u0
                                                        :type :expr
                                                        :at 1631125053948
                                                        :by |u0
                                                    :type :expr
                                                    :at 1631125040427
                                                    :by |u0
                                                :type :expr
                                                :at 1631125040427
                                                :by |u0
                                            :type :expr
                                            :at 1631125040427
                                            :by |u0
                                        :type :expr
                                        :at 1631124855274
                                        :by |u0
                                    :type :expr
                                    :at 1631124850351
                                    :by |u0
                                :type :expr
                                :at 1631124828282
                                :by |u0
                              |yr $ {}
                                :data $ {}
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1631126586625) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1631126590660) (:by |u0)
                                      |j $ {} (:text |:scale) (:type :leaf) (:at 1631126592836) (:by |u0)
                                    :type :expr
                                    :at 1631126587220
                                    :by |u0
                                  |n $ {}
                                    :data $ {}
                                      |T $ {} (:text |:factor) (:type :leaf) (:at 1631126603973) (:by |u0)
                                      |j $ {} (:text |2.5) (:type :leaf) (:at 1631126615436) (:by |u0)
                                    :type :expr
                                    :at 1631126601482
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:children) (:type :leaf) (:at 1631126594816) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1631126595419) (:by |u0)
                                          |j $ {}
                                            :data $ {}
                                              |yT $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:position)
                                                  |j $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                    :data $ {}
                                                      |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |[])
                                                      |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |200)
                                                      |r $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |200)
                                              |yj $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:fill-color)
                                                  |j $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                    :data $ {}
                                                      |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |[])
                                                      |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |40)
                                                      |r $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |80)
                                                      |v $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |80)
                                              |T $ {} (:text |{}) (:type :leaf) (:at 1631126598700) (:by |u0)
                                              |j $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:type)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:touch-area)
                                              |r $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:radius)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |10)
                                              |v $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:action)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |nil)
                                              |x $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:path)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |nil)
                                              |y $ {} (:type :expr) (:by |u0) (:at 1631126598700)
                                                :data $ {}
                                                  |T $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |:data)
                                                  |j $ {} (:type :leaf) (:by |u0) (:at 1631126598700) (:text |nil)
                                            :type :expr
                                            :at 1631126598700
                                            :by |u0
                                        :type :expr
                                        :at 1631126595129
                                        :by |u0
                                    :type :expr
                                    :at 1631126593446
                                    :by |u0
                                :type :expr
                                :at 1631126586096
                                :by |u0
                              |D $ {} (:text |[]) (:type :leaf) (:at 1620646420604) (:by |u0)
                              |T $ {}
                                :data $ {}
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1620627711428) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621179581008) (:by |u0)
                                      |j $ {} (:text |:rectangle) (:type :leaf) (:at 1620627718867) (:by |u0)
                                    :type :expr
                                    :at 1620627711597
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:position) (:type :leaf) (:at 1620639878089) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620639880221) (:by |u0)
                                          |j $ {} (:text |80) (:type :leaf) (:at 1620645742223) (:by |u0)
                                          |r $ {} (:text |100) (:type :leaf) (:at 1621851635366) (:by |u0)
                                        :type :expr
                                        :at 1620639878398
                                        :by |u0
                                    :type :expr
                                    :at 1620639791805
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:width) (:type :leaf) (:at 1620639884101) (:by |u0)
                                      |j $ {} (:text |100) (:type :leaf) (:at 1620640310337) (:by |u0)
                                    :type :expr
                                    :at 1620639882610
                                    :by |u0
                                  |x $ {}
                                    :data $ {}
                                      |T $ {} (:text |:height) (:type :leaf) (:at 1620639889950) (:by |u0)
                                      |j $ {} (:text |40) (:type :leaf) (:at 1620639890886) (:by |u0)
                                    :type :expr
                                    :at 1620639887708
                                    :by |u0
                                  |y $ {}
                                    :data $ {}
                                      |T $ {} (:text |:fill-color) (:type :leaf) (:at 1620645774831) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620645776531) (:by |u0)
                                          |j $ {} (:text |200) (:type :leaf) (:at 1620750615085) (:by |u0)
                                          |r $ {} (:text |80) (:type :leaf) (:at 1620645778697) (:by |u0)
                                          |v $ {} (:text |80) (:type :leaf) (:at 1620750620203) (:by |u0)
                                        :type :expr
                                        :at 1620645775064
                                        :by |u0
                                    :type :expr
                                    :at 1620645773055
                                    :by |u0
                                :type :expr
                                :at 1620627710698
                                :by |u0
                              |j $ {}
                                :data $ {}
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1620627711428) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621179583543) (:by |u0)
                                      |j $ {} (:text |:circle) (:type :leaf) (:at 1620646535357) (:by |u0)
                                    :type :expr
                                    :at 1620627711597
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:position) (:type :leaf) (:at 1620639878089) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620639880221) (:by |u0)
                                          |j $ {} (:text |120) (:type :leaf) (:at 1620646621724) (:by |u0)
                                          |r $ {} (:text |300) (:type :leaf) (:at 1620646623178) (:by |u0)
                                        :type :expr
                                        :at 1620639878398
                                        :by |u0
                                    :type :expr
                                    :at 1620639791805
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:radius) (:type :leaf) (:at 1620646541219) (:by |u0)
                                      |j $ {} (:text |40) (:type :leaf) (:at 1620646617536) (:by |u0)
                                    :type :expr
                                    :at 1620639882610
                                    :by |u0
                                  |y $ {}
                                    :data $ {}
                                      |T $ {} (:text |:fill-color) (:type :leaf) (:at 1620645774831) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620645776531) (:by |u0)
                                          |j $ {} (:text |0) (:type :leaf) (:at 1620645777947) (:by |u0)
                                          |r $ {} (:text |80) (:type :leaf) (:at 1620645778697) (:by |u0)
                                          |v $ {} (:text |70) (:type :leaf) (:at 1620645779299) (:by |u0)
                                        :type :expr
                                        :at 1620645775064
                                        :by |u0
                                    :type :expr
                                    :at 1620645773055
                                    :by |u0
                                :type :expr
                                :at 1620627710698
                                :by |u0
                              |r $ {}
                                :data $ {}
                                  |yT $ {}
                                    :data $ {}
                                      |T $ {} (:text |:align) (:type :leaf) (:at 1631032999464) (:by |u0)
                                      |j $ {} (:text |:center) (:type :leaf) (:at 1631033001972) (:by |u0)
                                    :type :expr
                                    :at 1631032997506
                                    :by |u0
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1620746959648) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621179595537) (:by |u0)
                                      |j $ {} (:text |:text) (:type :leaf) (:at 1621179593736) (:by |u0)
                                    :type :expr
                                    :at 1620746959648
                                    :by |u0
                                  |n $ {}
                                    :data $ {}
                                      |T $ {} (:text |:text) (:type :leaf) (:at 1620747000555) (:by |u0)
                                      |j $ {} (:text "|\"Demo") (:type :leaf) (:at 1620748320008) (:by |u0)
                                    :type :expr
                                    :at 1620746999435
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:position) (:type :leaf) (:at 1620746959648) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620746959648) (:by |u0)
                                          |j $ {} (:text |200) (:type :leaf) (:at 1620746959648) (:by |u0)
                                          |r $ {} (:text |100) (:type :leaf) (:at 1620748323021) (:by |u0)
                                        :type :expr
                                        :at 1620746959648
                                        :by |u0
                                    :type :expr
                                    :at 1620746959648
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:color) (:type :leaf) (:at 1620746959648) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620746959648) (:by |u0)
                                          |j $ {} (:text |0) (:type :leaf) (:at 1620748193554) (:by |u0)
                                          |r $ {} (:text |80) (:type :leaf) (:at 1620746959648) (:by |u0)
                                          |v $ {} (:text |100) (:type :leaf) (:at 1620748197964) (:by |u0)
                                        :type :expr
                                        :at 1620746959648
                                        :by |u0
                                    :type :expr
                                    :at 1620746959648
                                    :by |u0
                                  |x $ {}
                                    :data $ {}
                                      |T $ {} (:text |:size) (:type :leaf) (:at 1620746959648) (:by |u0)
                                      |j $ {} (:text |40) (:type :leaf) (:at 1620748203518) (:by |u0)
                                    :type :expr
                                    :at 1620746959648
                                    :by |u0
                                  |y $ {}
                                    :data $ {}
                                      |T $ {} (:text |:weight) (:type :leaf) (:at 1620747024799) (:by |u0)
                                      |j $ {} (:text "|\"300") (:type :leaf) (:at 1620747238140) (:by |u0)
                                    :type :expr
                                    :at 1620747023184
                                    :by |u0
                                :type :expr
                                :at 1620746959648
                                :by |u0
                              |v $ {}
                                :data $ {}
                                  |yT $ {}
                                    :data $ {}
                                      |T $ {} (:text |:stops) (:type :leaf) (:at 1620749132684) (:by |u0)
                                      |b $ {}
                                        :data $ {}
                                          |D $ {} (:text |->) (:type :leaf) (:at 1620750453530) (:by |u0)
                                          |T $ {}
                                            :data $ {}
                                              |T $ {} (:text |range) (:type :leaf) (:at 1620750447076) (:by |u0)
                                              |j $ {} (:text |20) (:type :leaf) (:at 1620750508466) (:by |u0)
                                            :type :expr
                                            :at 1620750446133
                                            :by |u0
                                          |j $ {}
                                            :data $ {}
                                              |T $ {} (:text |map) (:type :leaf) (:at 1620750454657) (:by |u0)
                                              |j $ {}
                                                :data $ {}
                                                  |T $ {} (:text |fn) (:type :leaf) (:at 1620750455591) (:by |u0)
                                                  |j $ {}
                                                    :data $ {}
                                                      |T $ {} (:text |i) (:type :leaf) (:at 1620750456449) (:by |u0)
                                                    :type :expr
                                                    :at 1620750455927
                                                    :by |u0
                                                  |r $ {}
                                                    :data $ {}
                                                      |T $ {} (:text |[]) (:type :leaf) (:at 1620750458778) (:by |u0)
                                                      |j $ {}
                                                        :data $ {}
                                                          |D $ {} (:text |*) (:type :leaf) (:at 1620750468524) (:by |u0)
                                                          |L $ {} (:text |80) (:type :leaf) (:at 1620750483109) (:by |u0)
                                                          |T $ {}
                                                            :data $ {}
                                                              |T $ {} (:text |cos) (:type :leaf) (:at 1620750461626) (:by |u0)
                                                              |j $ {}
                                                                :data $ {}
                                                                  |D $ {} (:text |*) (:type :leaf) (:at 1620750465253) (:by |u0)
                                                                  |L $ {} (:text |1.9) (:type :leaf) (:at 1620750502240) (:by |u0)
                                                                  |T $ {} (:text |i) (:type :leaf) (:at 1620750462383) (:by |u0)
                                                                :type :expr
                                                                :at 1620750464588
                                                                :by |u0
                                                            :type :expr
                                                            :at 1620750460481
                                                            :by |u0
                                                        :type :expr
                                                        :at 1620750467786
                                                        :by |u0
                                                      |r $ {}
                                                        :data $ {}
                                                          |D $ {} (:text |*) (:type :leaf) (:at 1620750468524) (:by |u0)
                                                          |L $ {} (:text |80) (:type :leaf) (:at 1620750485813) (:by |u0)
                                                          |T $ {}
                                                            :data $ {}
                                                              |T $ {} (:text |sin) (:type :leaf) (:at 1620750473710) (:by |u0)
                                                              |j $ {}
                                                                :data $ {}
                                                                  |D $ {} (:text |*) (:type :leaf) (:at 1620750465253) (:by |u0)
                                                                  |L $ {} (:text |1.9) (:type :leaf) (:at 1620750503958) (:by |u0)
                                                                  |T $ {} (:text |i) (:type :leaf) (:at 1620750462383) (:by |u0)
                                                                :type :expr
                                                                :at 1620750464588
                                                                :by |u0
                                                            :type :expr
                                                            :at 1620750460481
                                                            :by |u0
                                                        :type :expr
                                                        :at 1620750467786
                                                        :by |u0
                                                    :type :expr
                                                    :at 1620750458282
                                                    :by |u0
                                                :type :expr
                                                :at 1620750454994
                                                :by |u0
                                            :type :expr
                                            :at 1620750454123
                                            :by |u0
                                        :type :expr
                                        :at 1620750451560
                                        :by |u0
                                    :type :expr
                                    :at 1620749127661
                                    :by |u0
                                  |yj $ {}
                                    :data $ {}
                                      |T $ {} (:text |:join) (:type :leaf) (:at 1621179677746) (:by |u0)
                                      |j $ {} (:text |:round) (:type :leaf) (:at 1620749173797) (:by |u0)
                                    :type :expr
                                    :at 1620749156160
                                    :by |u0
                                  |yr $ {}
                                    :data $ {}
                                      |T $ {} (:text |:cap) (:type :leaf) (:at 1621179679843) (:by |u0)
                                      |j $ {} (:text |:round) (:type :leaf) (:at 1620749173797) (:by |u0)
                                    :type :expr
                                    :at 1620749156160
                                    :by |u0
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1620749096272) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621179590472) (:by |u0)
                                      |j $ {} (:text |:polyline) (:type :leaf) (:at 1620749100422) (:by |u0)
                                    :type :expr
                                    :at 1620749096596
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:position) (:type :leaf) (:at 1620749102380) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620749103596) (:by |u0)
                                          |j $ {} (:text |400) (:type :leaf) (:at 1620750512326) (:by |u0)
                                          |r $ {} (:text |200) (:type :leaf) (:at 1620749279337) (:by |u0)
                                        :type :expr
                                        :at 1620749102892
                                        :by |u0
                                    :type :expr
                                    :at 1620749101002
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:color) (:type :leaf) (:at 1620749110523) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1620749110997) (:by |u0)
                                          |j $ {} (:text |0) (:type :leaf) (:at 1620749112026) (:by |u0)
                                          |r $ {} (:text |0) (:type :leaf) (:at 1620749112285) (:by |u0)
                                          |v $ {} (:text |100) (:type :leaf) (:at 1620749112807) (:by |u0)
                                          |x $ {} (:text |1) (:type :leaf) (:at 1620749273737) (:by |u0)
                                        :type :expr
                                        :at 1620749110742
                                        :by |u0
                                    :type :expr
                                    :at 1620749108089
                                    :by |u0
                                  |x $ {}
                                    :data $ {}
                                      |T $ {} (:text |:skip-first?) (:type :leaf) (:at 1620749117647) (:by |u0)
                                      |j $ {} (:text |true) (:type :leaf) (:at 1620749118430) (:by |u0)
                                    :type :expr
                                    :at 1620749114230
                                    :by |u0
                                  |y $ {}
                                    :data $ {}
                                      |T $ {} (:text |:width) (:type :leaf) (:at 1621179691313) (:by |u0)
                                      |j $ {} (:text |2) (:type :leaf) (:at 1620749127107) (:by |u0)
                                    :type :expr
                                    :at 1620749121022
                                    :by |u0
                                :type :expr
                                :at 1620749095558
                                :by |u0
                              |x $ {}
                                :data $ {}
                                  |yT $ {}
                                    :data $ {}
                                      |T $ {} (:text |:fill-color) (:type :leaf) (:at 1621018727443) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1621018730060) (:by |u0)
                                          |j $ {} (:text |40) (:type :leaf) (:at 1631124908378) (:by |u0)
                                          |r $ {} (:text |80) (:type :leaf) (:at 1621018734828) (:by |u0)
                                          |v $ {} (:text |80) (:type :leaf) (:at 1621018735527) (:by |u0)
                                        :type :expr
                                        :at 1621018728569
                                        :by |u0
                                    :type :expr
                                    :at 1621018699698
                                    :by |u0
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1621018608336) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621179598941) (:by |u0)
                                      |j $ {} (:text |:touch-area) (:type :leaf) (:at 1621018614481) (:by |u0)
                                    :type :expr
                                    :at 1621018608877
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:radius) (:type :leaf) (:at 1621018638257) (:by |u0)
                                      |j $ {} (:text |10) (:type :leaf) (:at 1621018750192) (:by |u0)
                                    :type :expr
                                    :at 1621018630122
                                    :by |u0
                                  |t $ {}
                                    :data $ {}
                                      |T $ {} (:text |:action) (:type :leaf) (:at 1621018681876) (:by |u0)
                                      |j $ {} (:text |nil) (:type :leaf) (:at 1621018682479) (:by |u0)
                                    :type :expr
                                    :at 1621018680593
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:path) (:type :leaf) (:at 1621018679268) (:by |u0)
                                      |j $ {} (:text |nil) (:type :leaf) (:at 1621018679934) (:by |u0)
                                    :type :expr
                                    :at 1621018641655
                                    :by |u0
                                  |x $ {}
                                    :data $ {}
                                      |T $ {} (:text |:data) (:type :leaf) (:at 1621018686471) (:by |u0)
                                      |j $ {} (:text |nil) (:type :leaf) (:at 1621018688803) (:by |u0)
                                    :type :expr
                                    :at 1621018683368
                                    :by |u0
                                  |y $ {}
                                    :data $ {}
                                      |T $ {} (:text |:position) (:type :leaf) (:at 1621018692730) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1621018694278) (:by |u0)
                                          |j $ {} (:text |200) (:type :leaf) (:at 1621018697029) (:by |u0)
                                          |r $ {} (:text |200) (:type :leaf) (:at 1621018698502) (:by |u0)
                                        :type :expr
                                        :at 1621018693470
                                        :by |u0
                                    :type :expr
                                    :at 1621018689580
                                    :by |u0
                                :type :expr
                                :at 1621018607089
                                :by |u0
                              |y $ {}
                                :data $ {}
                                  |T $ {} (:text |{}) (:type :leaf) (:at 1621096073407) (:by |u0)
                                  |j $ {}
                                    :data $ {}
                                      |T $ {} (:text |:type) (:type :leaf) (:at 1621179601837) (:by |u0)
                                      |j $ {} (:text |:key-listener) (:type :leaf) (:at 1621096080661) (:by |u0)
                                    :type :expr
                                    :at 1621096073864
                                    :by |u0
                                  |r $ {}
                                    :data $ {}
                                      |T $ {} (:text |:key) (:type :leaf) (:at 1621096084326) (:by |u0)
                                      |j $ {} (:text "|\"D") (:type :leaf) (:at 1621096085295) (:by |u0)
                                    :type :expr
                                    :at 1621096082650
                                    :by |u0
                                  |v $ {}
                                    :data $ {}
                                      |T $ {} (:text |:action) (:type :leaf) (:at 1621096091465) (:by |u0)
                                      |j $ {} (:text |:keyboard) (:type :leaf) (:at 1621096095944) (:by |u0)
                                    :type :expr
                                    :at 1621096088220
                                    :by |u0
                                  |x $ {}
                                    :data $ {}
                                      |T $ {} (:text |:path) (:type :leaf) (:at 1621096099964) (:by |u0)
                                      |j $ {}
                                        :data $ {}
                                          |T $ {} (:text |[]) (:type :leaf) (:at 1621096102313) (:by |u0)
                                          |j $ {} (:text |:k) (:type :leaf) (:at 1621096103571) (:by |u0)
                                        :type :expr
                                        :at 1621096101456
                                        :by |u0
                                    :type :expr
                                    :at 1621096098024
                                    :by |u0
                                  |y $ {}
                                    :data $ {}
                                      |T $ {} (:text |:data) (:type :leaf) (:at 1621096105726) (:by |u0)
                                      |j $ {} (:text |:data) (:type :leaf) (:at 1621096106919) (:by |u0)
                                    :type :expr
                                    :at 1621096104591
                                    :by |u0
                                :type :expr
                                :at 1621096072396
                                :by |u0
                            :type :expr
                            :at 1620646419925
                            :by |u0
                        :type :expr
                        :at 1620646415182
                        :by |u0
                    :type :expr
                    :at 1620646403375
                    :by |u0
                :type :expr
                :at 1620627697351
                :by |u0
            :type :expr
            :at 1620627695917
            :by |u0
        :proc $ {}
          :data $ {}
          :type :expr
          :at 1620576367501
          :by |u0
        :configs $ {}
        :ns $ {}
          :data $ {}
            |T $ {} (:text |ns) (:type :leaf) (:at 1620576367501) (:by |u0)
            |j $ {} (:text |app.main) (:type :leaf) (:at 1620576367501) (:by |u0)
          :type :expr
          :at 1620576367501
          :by |u0
  :users $ {}
    |u0 $ {} (:avatar nil) (:name |chen) (:nickname |chen) (:id |u0) (:theme :star-trail) (:password |d41d8cd98f00b204e9800998ecf8427e)
