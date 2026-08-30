{}
  :schema-version 1
  :feature 'window-lifecycle
  :doc "|Expose one configured Paint window as a nominal Calcit boundary, with runtime requests serialized by the native event loop."
  :roots $ #{} 'calcit-paint.core/launch-canvas-with-options! 'calcit-paint.core/set-window-title! 'calcit-paint.core/request-window-size! 'calcit-paint.core/close-window!
  :definitions $ {}
    'calcit-paint.core/WindowOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Strict startup configuration for the single Paint window."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct WindowOptions (:title 'String) (:width 'Number) (:height 'Number) (:min-width 'Number) (:min-height 'Number) (:resizable? 'Bool)
    'calcit-paint.core/launch-canvas-with-options! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Launch the blocking single-window event loop with nominal startup options."
      :params $ [] 'options 'cb
      :schema $ :: 'Fn
        {}
          :args $ [] 'calcit-paint.core/WindowOptions
            :: 'Fn $ {} (:return 'R)
              :args $ [] 'Dynamic
          :return 'Unit
          :generics $ [] 'R
    'calcit-paint.core/set-window-title! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Queue a title update for serialized application on the active event loop."
      :params $ [] 'title
      :schema $ :: 'Fn
        {} (:return 'Unit)
          :args $ [] 'String
    'calcit-paint.core/request-window-size! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Queue a positive finite logical-size request for the active Paint window."
      :params $ [] 'width 'height
      :schema $ :: 'Fn
        {} (:return 'Unit)
          :args $ [] 'Number 'Number
    'calcit-paint.core/close-window! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Queue an orderly close request for the active Paint window."
      :params $ []
      :schema $ :: 'Fn
        {} (:return 'Unit)
          :args $ []
  :edges $ #{}
    :: :type 'calcit-paint.core/launch-canvas-with-options! 'calcit-paint.core/WindowOptions
