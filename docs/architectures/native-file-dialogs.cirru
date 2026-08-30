{}
  :schema-version 1
  :feature 'native-file-dialogs
  :doc "|Queue native open/save dialogs without blocking the Paint event loop, then deliver a typed terminal result. / 原生打开与保存对话框通过队列异步运行，并以强类型终态结果回传，不阻塞 Paint 事件循环。"
  :roots $ #{} 'calcit-paint.core/open-file-dialog! 'calcit-paint.core/save-file-dialog! 'calcit-paint.core/PaintFileDialogEvent
  :definitions $ {}
    'calcit-paint.core/PaintFileDialogFilter $ {}
      :mode :ensure
      :kind :data
      :doc "|One native file dialog filter with a human label and extensions without leading dots. / 原生文件对话框过滤器，含人类可读标签与不带前导点的扩展名。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileDialogFilter (:name 'String) (:extensions $ :: 'List 'String)
    'calcit-paint.core/PaintFileDialogOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Strict native dialog request options; request-id is echoed by the terminal result event. / 严格原生对话框请求配置；request-id 会在终态结果事件中原样返回。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileDialogOptions (:request-id 'String)
          :title $ :: 'Option 'String
          :directory $ :: 'Option 'FsPath
          :file-name $ :: 'Option 'String
          :filters $ :: 'List 'calcit-paint.core/PaintFileDialogFilter
    'calcit-paint.core/PaintFileDialogEventWire $ {}
      :mode :ensure
      :kind :data
      :doc "|Internal UTF-8 native dialog result decoded before constructing a nominal FsPath. / 内部 UTF-8 原生对话框结果，在构造 nominal FsPath 前解码。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileDialogEventWire (:request-id 'String) (:operation 'Tag) (:status 'Tag)
          :path $ :: 'Option 'String
          :error $ :: 'Option 'String
    'calcit-paint.core/PaintFileDialogEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed terminal native-dialog result. Status is :selected, :cancelled, or :failed. / 强类型原生对话框终态结果。status 为 :selected、:cancelled 或 :failed。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileDialogEvent (:request-id 'String) (:operation 'Tag) (:status 'Tag)
          :path $ :: 'Option 'FsPath
          :error $ :: 'Option 'String
    'calcit-paint.core/open-file-dialog! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'options
      :doc "|Queue one native open-file dialog; its result is delivered later as :file-dialog-result. / 请求一个原生打开文件对话框；结果稍后以 :file-dialog-result 送达。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.core/PaintFileDialogOptions
        :return 'Unit
      :code $ quote
        defn open-file-dialog! (options)
          &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |open_file_dialog options
          , &unit
    'calcit-paint.core/save-file-dialog! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'options
      :doc "|Queue one native save-file dialog; its result is delivered later as :file-dialog-result. / 请求一个原生保存文件对话框；结果稍后以 :file-dialog-result 送达。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.core/PaintFileDialogOptions
        :return 'Unit
      :code $ quote
        defn save-file-dialog! (options)
          &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |save_file_dialog options
          , &unit
  :edges $ #{}
    :: :type 'calcit-paint.core/PaintFileDialogOptions 'calcit-paint.core/PaintFileDialogFilter
