{}
  :schema-version 1
  :feature 'text-clipboard
  :doc "|Expose serialized cross-platform UTF-8 clipboard effects through typed Calcit wrappers. / 通过强类型 Calcit wrapper 暴露串行跨平台 UTF-8 剪贴板副作用。"
  :roots $ #{} 'calcit-paint.core/read-clipboard-text! 'calcit-paint.core/write-clipboard-text!
  :definitions $ {}
    'calcit-paint.core/read-clipboard-text! $ {}
      :mode :ensure
      :kind :fn
      :params $ []
      :doc "|Read UTF-8 text from the serialized system clipboard. / 从串行系统剪贴板读取 UTF-8 文本。"
      :schema $ :: 'Fn $ {}
        :args $ []
        :return 'String
      :code $ quote
        defn read-clipboard-text! ()
          &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |read_clipboard_text
    'calcit-paint.core/write-clipboard-text! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'text
      :doc "|Write UTF-8 text to the serialized system clipboard. / 向串行系统剪贴板写入 UTF-8 文本。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'String
        :return 'Unit
      :code $ quote
        defn write-clipboard-text! (text)
          &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |write_clipboard_text text
          , &unit
  :edges $ #{}
