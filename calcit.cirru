
{} (:about "|Machine-generated snapshot. Do not edit directly — changes will be overwritten. Use `calcit query` to inspect and `calcit edit`/`calcit tree` to modify. Run `calcit docs agents --full` first. Manual edits must follow format and schema conventions, then run `calcit edit format`.") (:package |calcit-paint)
  :entries $ {}
    :default $ {} (:description |) (:init-fn 'calcit-paint.main/main!) (:mode :native) (:reload-fn 'calcit-paint.main/reload!)
      :feature-policy $ {}
      :modules $ []
      :type-slots $ {}
  :files $ {}
    'calcit-paint.core $ %{} 'FileEntry
      :defs $ {}
        'PaintAccessibilityActionEvent $ %{} 'CodeEntry (:doc "|Typed accessibility action from the platform semantic tree. Operation is :focus or :activate. / 来自平台语义树的强类型可访问性动作。operation 为 :focus 或 :activate。")
          :code $ quote
            defstruct PaintAccessibilityActionEvent (:id 'String) (:operation 'Tag) (:target 'calcit-paint.core/PaintTarget)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintEvent $ %{} 'CodeEntry (:doc "|Nominal exhaustive event protocol for typed Paint callbacks, including system theme observations. / 用于强类型 Paint callback 的 nominal 穷尽事件协议，包含系统主题观测。")
          :code $ quote
            defenum PaintEvent (:ready) (:frame 'calcit-paint.core/PaintFrameEvent) (:mouse-down 'calcit-paint.core/PaintPointerEvent) (:mouse-up 'calcit-paint.core/PaintPointerEvent) (:mouse-move 'calcit-paint.core/PaintPointerEvent) (:mouse-leave 'calcit-paint.core/PaintPointerEvent) (:mouse-wheel 'calcit-paint.core/PaintPointerEvent) (:pointer-enter 'calcit-paint.core/PaintPointerEvent) (:pointer-leave 'calcit-paint.core/PaintPointerEvent) (:pointer-cancel 'calcit-paint.core/PaintPointerEvent) (:key-down 'calcit-paint.core/PaintKeyboardEvent) (:key-up 'calcit-paint.core/PaintKeyboardEvent) (:focus-in 'calcit-paint.core/PaintFocusEvent) (:focus-out 'calcit-paint.core/PaintFocusEvent) (:ime-enabled 'calcit-paint.core/PaintTextInputEvent) (:ime-disabled 'calcit-paint.core/PaintTextInputEvent) (:composition-start 'calcit-paint.core/PaintTextInputEvent) (:composition-update 'calcit-paint.core/PaintTextInputEvent) (:composition-end 'calcit-paint.core/PaintTextInputEvent) (:text-input 'calcit-paint.core/PaintTextInputEvent) (:file-hover 'calcit-paint.core/PaintFileEvent) (:file-drop 'calcit-paint.core/PaintFileEvent) (:file-hover-cancel 'calcit-paint.core/PaintFileHoverCancelEvent) (:file-dialog-result 'calcit-paint.core/PaintFileDialogEvent) (:accessibility-action 'calcit-paint.core/PaintAccessibilityActionEvent) (:window-focus) (:window-blur) (:resize 'calcit-paint.core/PaintWindowMetricsEvent) (:scale-factor 'calcit-paint.core/PaintWindowMetricsEvent) (:window-theme 'calcit-paint.core/PaintWindowThemeEvent) (:window-title-applied 'calcit-paint.core/PaintWindowTitleEvent) (:window-size-request 'calcit-paint.core/PaintWindowSizeEvent) (:window-close 'calcit-paint.core/PaintWindowCloseEvent)
          :examples $ []
          :schema $ :: 'EnumDef
        'PaintEventFfi $ %{} 'CodeEntry (:doc "|Internal generic envelope received from the native typed-event transport.")
          :code $ quote
            defenum PaintEventFfi ([] 'Raw) (:ready) (:frame 'Raw) (:mouse-down 'Raw) (:mouse-up 'Raw) (:mouse-move 'Raw) (:mouse-leave 'Raw) (:mouse-wheel 'Raw) (:pointer-enter 'Raw) (:pointer-leave 'Raw) (:pointer-cancel 'Raw) (:key-down 'Raw) (:key-up 'Raw) (:focus-in 'Raw) (:focus-out 'Raw) (:ime-enabled 'Raw) (:ime-disabled 'Raw) (:composition-start 'Raw) (:composition-update 'Raw) (:composition-end 'Raw) (:text-input 'Raw) (:file-hover 'Raw) (:file-drop 'Raw) (:file-hover-cancel 'Raw) (:file-dialog-result 'Raw) (:accessibility-action 'Raw) (:window-focus) (:window-blur) (:resize 'Raw) (:scale-factor 'Raw) (:window-theme 'Raw) (:window-title-applied 'Raw) (:window-size-request 'Raw) (:window-close 'Raw)
          :examples $ []
          :schema $ :: 'EnumDef
        'PaintFileDialogEvent $ %{} 'CodeEntry (:doc "|Typed terminal native-dialog result. Status is :selected, :cancelled, or :failed. / 强类型原生对话框终态结果。status 为 :selected、:cancelled 或 :failed。")
          :code $ quote
            defstruct PaintFileDialogEvent (:request-id 'String) (:operation 'Tag) (:status 'Tag)
              :path $ :: 'Option 'FsPath
              :error $ :: 'Option 'String
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFileDialogEventWire $ %{} 'CodeEntry (:doc "|Internal UTF-8 native dialog result decoded before constructing a nominal FsPath. / 内部 UTF-8 原生对话框结果，在构造 nominal FsPath 前解码。")
          :code $ quote
            defstruct PaintFileDialogEventWire (:request-id 'String) (:operation 'Tag) (:status 'Tag)
              :path $ :: 'Option 'String
              :error $ :: 'Option 'String
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFileDialogFilter $ %{} 'CodeEntry (:doc "|One native file dialog filter with a human label and extensions without leading dots. / 原生文件对话框过滤器，含人类可读标签与不带前导点的扩展名。")
          :code $ quote
            defstruct PaintFileDialogFilter (:name 'String)
              :extensions $ :: 'List 'String
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFileDialogOptions $ %{} 'CodeEntry (:doc "|Strict native dialog request options; request-id is echoed by the terminal result event. / 严格原生对话框请求配置；request-id 会在终态结果事件中原样返回。")
          :code $ quote
            defstruct PaintFileDialogOptions (:request-id 'String)
              :title $ :: 'Option 'String
              :directory $ :: 'Option 'FsPath
              :file-name $ :: 'Option 'String
              :filters $ :: 'List 'calcit-paint.core/PaintFileDialogFilter
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFileEvent $ %{} 'CodeEntry (:doc "|Typed file hover/drop payload with a nominal Calcit filesystem path.")
          :code $ quote
            defstruct PaintFileEvent (:path 'FsPath) (:x 'Number) (:y 'Number) (:modifiers 'calcit-paint.core/PaintModifiers)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFileEventWire $ %{} 'CodeEntry (:doc "|Internal UTF-8 wire payload decoded before constructing a nominal FsPath.")
          :code $ quote
            defstruct PaintFileEventWire (:path 'String) (:x 'Number) (:y 'Number) (:modifiers 'calcit-paint.core/PaintModifiers)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFileHoverCancelEvent $ %{} 'CodeEntry (:doc "|Typed file-hover cancellation payload without a fabricated path.")
          :code $ quote
            defstruct PaintFileHoverCancelEvent (:x 'Number) (:y 'Number) (:modifiers 'calcit-paint.core/PaintModifiers)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFocusEvent $ %{} 'CodeEntry (:doc "|Typed focus transition payload.")
          :code $ quote
            defstruct PaintFocusEvent (:focus-id 'String)
              :related-focus-id $ :: 'Option 'String
              :reason 'Tag
              :target 'calcit-paint.core/PaintTarget
          :examples $ []
          :schema $ :: 'StructDef
        'PaintFrameEvent $ %{} 'CodeEntry (:doc "|Typed on-demand animation frame payload.")
          :code $ quote
            defstruct PaintFrameEvent (:frame 'Number) (:timestamp-ms 'Number) (:delta-ms 'Number) (:width 'Number) (:height 'Number) (:scale-factor 'Number)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintKeyboardEvent $ %{} 'CodeEntry (:doc "|Typed logical and physical keyboard payload.")
          :code $ quote
            defstruct PaintKeyboardEvent (:key-code 'Number) (:physical-key 'String) (:name 'String) (:modifiers 'calcit-paint.core/PaintModifiers)
              :focus-id $ :: 'Option 'String
              :shortcut? $ :: 'Option 'Bool
              :target 'calcit-paint.core/PaintTarget
          :examples $ []
          :schema $ :: 'StructDef
        'PaintModifiers $ %{} 'CodeEntry (:doc "|Closed keyboard and pointer modifier state.")
          :code $ quote
            defstruct PaintModifiers (:shift? 'Bool) (:control? 'Bool) (:alt? 'Bool) (:super? 'Bool)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintPointerEvent $ %{} 'CodeEntry (:doc "|Typed pointer, mouse, hover, cancellation, and wheel payload.")
          :code $ quote
            defstruct PaintPointerEvent (:x 'Number) (:y 'Number) (:clicks 'Number) (:modifiers 'calcit-paint.core/PaintModifiers) (:target 'calcit-paint.core/PaintTarget)
              :dx $ :: 'Option 'Number
              :dy $ :: 'Option 'Number
              :button $ :: 'Option 'Tag
              :button-id $ :: 'Option 'Number
              :cursor $ :: 'Option 'Tag
              :captured? $ :: 'Option 'Bool
              :cancelled? $ :: 'Option 'Bool
              :reason $ :: 'Option 'Tag
              :unit $ :: 'Option 'Tag
          :examples $ []
          :schema $ :: 'StructDef
        'PaintTarget $ %{} 'CodeEntry (:doc "|Application-defined target values kept at one explicit open boundary.")
          :code $ quote
            defstruct PaintTarget
              :action $ :: 'Option 'Dynamic
              :path $ :: 'Option 'Dynamic
              :data $ :: 'Option 'Dynamic
          :examples $ []
          :schema $ :: 'StructDef
        'PaintTextInputEvent $ %{} 'CodeEntry (:doc "|Typed IME composition and committed text payload.")
          :code $ quote
            defstruct PaintTextInputEvent (:focus-id 'String) (:text 'String)
              :cursor-start $ :: 'Option 'Number
              :cursor-end $ :: 'Option 'Number
              :cancelled? $ :: 'Option 'Bool
              :target 'calcit-paint.core/PaintTarget
          :examples $ []
          :schema $ :: 'StructDef
        'PaintWindowCloseEvent $ %{} 'CodeEntry (:doc "|Typed exactly-once window close payload.")
          :code $ quote
            defstruct PaintWindowCloseEvent $ :reason 'Tag
          :examples $ []
          :schema $ :: 'StructDef
        'PaintWindowMetricsEvent $ %{} 'CodeEntry (:doc "|Typed logical window metrics and scale payload.")
          :code $ quote
            defstruct PaintWindowMetricsEvent (:width 'Number) (:height 'Number) (:scale-factor 'Number)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintWindowSizeEvent $ %{} 'CodeEntry (:doc "|Typed logical-size request acknowledgement.")
          :code $ quote
            defstruct PaintWindowSizeEvent (:status 'Tag) (:requested-width 'Number) (:requested-height 'Number)
              :actual-width $ :: 'Option 'Number
              :actual-height $ :: 'Option 'Number
              :matched? $ :: 'Option 'Bool
              :scale-factor 'Number
          :examples $ []
          :schema $ :: 'StructDef
        'PaintWindowThemeEvent $ %{} 'CodeEntry (:doc "|Typed system theme payload: :light, :dark, or :unknown, plus whether it is the initial observation. / 强类型系统主题 payload：:light、:dark 或 :unknown，并标明是否为初始观测。")
          :code $ quote
            defstruct PaintWindowThemeEvent (:theme 'Tag) (:initial? 'Bool)
          :examples $ []
          :schema $ :: 'StructDef
        'PaintWindowTitleEvent $ %{} 'CodeEntry (:doc "|Typed applied window-title acknowledgement.")
          :code $ quote
            defstruct PaintWindowTitleEvent $ :title 'String
          :examples $ []
          :schema $ :: 'StructDef
        'WindowOptions $ %{} 'CodeEntry (:doc "|Strict startup configuration for the single Paint window.")
          :code $ quote
            defstruct WindowOptions (:title 'String) (:width 'Number) (:height 'Number) (:min-width 'Number) (:min-height 'Number) (:resizable? 'Bool)
          :examples $ []
          :schema $ :: 'StructDef
        'blur! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn blur! ()
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |clear_focus
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'close-window! $ %{} 'CodeEntry (:doc "|Queue an orderly close request for the active Paint window.")
          :code $ quote
            defn close-window! ()
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |close_window
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'focus! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn focus! (id)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |request_focus id
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'String
        'focused? $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn focused? (id)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |focused id
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Bool)
              :args $ [] 'String
        'launch-canvas! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn launch-canvas! (cb)
              &blocking-dylib-edn-fn (get-dylib-path |/dylibs/libcalcit_paint) |launch_canvas $ fn (event) (cb event) :handled
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
                :: 'Fn $ {} (:return 'R)
                  :args $ [] 'Dynamic
              :generics $ [] 'R
        'launch-canvas-typed! $ %{} 'CodeEntry (:doc "|Launch the configured single Paint window with a nominal PaintEvent callback.")
          :code $ quote
            defn launch-canvas-typed! (options cb)
              &blocking-dylib-edn-fn (get-dylib-path |/dylibs/libcalcit_paint) |launch_canvas_typed options $ fn (event)
                cb $ paint-event-from-ffi event
                , :handled
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/WindowOptions
                :: 'Fn $ {} (:return 'R)
                  :args $ [] 'calcit-paint.core/PaintEvent
              :generics $ [] 'R
        'launch-canvas-with-options! $ %{} 'CodeEntry (:doc "|Launch the blocking single-window event loop with nominal startup options.")
          :code $ quote
            defn launch-canvas-with-options! (options cb)
              &blocking-dylib-edn-fn (get-dylib-path |/dylibs/libcalcit_paint) |launch_canvas_with_options options $ fn (event) (cb event) :handled
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/WindowOptions
                :: 'Fn $ {} (:return 'R)
                  :args $ [] 'Dynamic
              :generics $ [] 'R
        'measure-paragraph! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn measure-paragraph! (data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |measure_paragraph data
          :examples $ []
          :schema $ :: 'Fn
            {}
              :args $ [] (:: 'Map 'Tag 'Dynamic)
              :return $ :: 'Map 'Tag 'Number
        'measure-text! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn measure-text! (data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |measure_text data
          :examples $ []
          :schema $ :: 'Fn
            {}
              :args $ [] (:: 'Map 'Tag 'Dynamic)
              :return $ :: 'Map 'Tag 'Number
        'open-file-dialog! $ %{} 'CodeEntry (:doc "|Queue one native open-file dialog; its result is delivered later as :file-dialog-result. / 请求一个原生打开文件对话框；结果稍后以 :file-dialog-result 送达。")
          :code $ quote
            defn open-file-dialog! (options)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |open_file_dialog options
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/PaintFileDialogOptions
        'paint-event-from-ffi $ %{} 'CodeEntry (:doc "|Strictly decode one native typed-event envelope into the public nominal protocol, validating system theme tags. / 严格将原生强类型事件 envelope 解码为公开 nominal 协议，并校验系统主题 tag。")
          :code $ quote
            defn paint-event-from-ffi (event)
              match event
                (:ready) (PaintEvent :ready)
                (:frame payload)
                  PaintEvent :frame $ decode-map-as payload PaintFrameEvent
                (:mouse-down payload)
                  PaintEvent :mouse-down $ decode-map-as payload PaintPointerEvent
                (:mouse-up payload)
                  PaintEvent :mouse-up $ decode-map-as payload PaintPointerEvent
                (:mouse-move payload)
                  PaintEvent :mouse-move $ decode-map-as payload PaintPointerEvent
                (:mouse-leave payload)
                  PaintEvent :mouse-leave $ decode-map-as payload PaintPointerEvent
                (:mouse-wheel payload)
                  PaintEvent :mouse-wheel $ decode-map-as payload PaintPointerEvent
                (:pointer-enter payload)
                  PaintEvent :pointer-enter $ decode-map-as payload PaintPointerEvent
                (:pointer-leave payload)
                  PaintEvent :pointer-leave $ decode-map-as payload PaintPointerEvent
                (:pointer-cancel payload)
                  PaintEvent :pointer-cancel $ decode-map-as payload PaintPointerEvent
                (:key-down payload)
                  PaintEvent :key-down $ decode-map-as payload PaintKeyboardEvent
                (:key-up payload)
                  PaintEvent :key-up $ decode-map-as payload PaintKeyboardEvent
                (:focus-in payload)
                  PaintEvent :focus-in $ decode-map-as payload PaintFocusEvent
                (:focus-out payload)
                  PaintEvent :focus-out $ decode-map-as payload PaintFocusEvent
                (:ime-enabled payload)
                  PaintEvent :ime-enabled $ decode-map-as payload PaintTextInputEvent
                (:ime-disabled payload)
                  PaintEvent :ime-disabled $ decode-map-as payload PaintTextInputEvent
                (:composition-start payload)
                  PaintEvent :composition-start $ decode-map-as payload PaintTextInputEvent
                (:composition-update payload)
                  PaintEvent :composition-update $ decode-map-as payload PaintTextInputEvent
                (:composition-end payload)
                  PaintEvent :composition-end $ decode-map-as payload PaintTextInputEvent
                (:text-input payload)
                  PaintEvent :text-input $ decode-map-as payload PaintTextInputEvent
                (:file-hover payload)
                  let
                      wire $ decode-map-as payload PaintFileEventWire
                    PaintEvent :file-hover $ PaintFileEvent :path
                      fs:path $ :path wire
                      , :x (:x wire) :y (:y wire) :modifiers (:modifiers wire)
                (:file-drop payload)
                  let
                      wire $ decode-map-as payload PaintFileEventWire
                    PaintEvent :file-drop $ PaintFileEvent :path
                      fs:path $ :path wire
                      , :x (:x wire) :y (:y wire) :modifiers (:modifiers wire)
                (:file-hover-cancel payload)
                  PaintEvent :file-hover-cancel $ decode-map-as payload PaintFileHoverCancelEvent
                (:file-dialog-result payload)
                  let
                      wire $ decode-map-as payload PaintFileDialogEventWire
                      path $ :path wire
                      error $ :error wire
                      path-for-event $ match path
                        (:none) %none
                        (:some value)
                          %some $ fs:path value
                      result $ PaintFileDialogEvent :request-id (:request-id wire) :operation (:operation wire) :status (:status wire) :path path-for-event :error error
                    do
                      case-default (:operation result)
                        raise $ str "|unsupported typed file dialog operation: " (:operation result)
                        :open &unit
                        :save &unit
                      case-default (:status result)
                        raise $ str "|unsupported typed file dialog status: " (:status result)
                        :selected $ PaintEvent :file-dialog-result result
                        :cancelled $ PaintEvent :file-dialog-result result
                        :failed $ PaintEvent :file-dialog-result result
                (:accessibility-action payload)
                  let
                      action $ decode-map-as payload PaintAccessibilityActionEvent
                    case-default (:operation action)
                      raise $ str "|unsupported typed accessibility operation: " (:operation action)
                      :focus $ PaintEvent :accessibility-action action
                      :activate $ PaintEvent :accessibility-action action
                (:window-focus) (PaintEvent :window-focus)
                (:window-blur) (PaintEvent :window-blur)
                (:resize payload)
                  PaintEvent :resize $ decode-map-as payload PaintWindowMetricsEvent
                (:scale-factor payload)
                  PaintEvent :scale-factor $ decode-map-as payload PaintWindowMetricsEvent
                (:window-theme payload)
                  let
                      theme-event $ decode-map-as payload PaintWindowThemeEvent
                    case-default (:theme theme-event)
                      raise $ str "|unsupported typed system theme: " (:theme theme-event)
                      :light $ PaintEvent :window-theme theme-event
                      :dark $ PaintEvent :window-theme theme-event
                      :unknown $ PaintEvent :window-theme theme-event
                (:window-title-applied payload)
                  PaintEvent :window-title-applied $ decode-map-as payload PaintWindowTitleEvent
                (:window-size-request payload)
                  PaintEvent :window-size-request $ decode-map-as payload PaintWindowSizeEvent
                (:window-close payload)
                  PaintEvent :window-close $ decode-map-as payload PaintWindowCloseEvent
          :examples $ []
            quote $ let
                event $ paint-event-from-ffi
                  PaintEventFfi :frame $ {} (:frame 7) (:timestamp-ms 32) (:delta-ms 16) (:width 800) (:height 600) (:scale-factor 2)
              assert-type event 'calcit-paint.core/PaintEvent
              match event
                (:frame payload)
                  assert= 7 $ :frame payload
                _ $ raise |expected-frame-event
            quote $ paint-event-from-ffi
              PaintEventFfi :file-hover $ {} (:path |assets/demo.png) (:x 24) (:y 36)
                :modifiers $ {} (:shift? false) (:control? false) (:alt? false) (:super? false)
          :schema $ :: 'Fn
            {} (:return 'calcit-paint.core/PaintEvent)
              :args $ []
                :: 'calcit-paint.core/PaintEventFfi $ :: 'Map 'Tag 'Dynamic
          :tests $ []
            %{} 'TestEntry (:name |decodes-frame-payload)
              :code $ quote
                let
                    event $ paint-event-from-ffi
                      PaintEventFfi :frame $ {} (:frame 7) (:timestamp-ms 32) (:delta-ms 16) (:width 800) (:height 600) (:scale-factor 2)
                  assert-type event 'calcit-paint.core/PaintEvent
                  match event
                    (:frame payload)
                      assert= 7 $ :frame payload
                    _ $ raise |expected-frame-event
              :tags $ #{} :unit
            %{} 'TestEntry (:name |decodes-pointer-target)
              :code $ quote
                let
                    event $ paint-event-from-ffi
                      PaintEventFfi :mouse-down $ {} (:x 12) (:y 8) (:clicks 1)
                        :modifiers $ {} (:shift? false) (:control? false) (:alt? false) (:super? false)
                        :target $ {} (:action :select)
                        :button :primary
                  assert-type event 'calcit-paint.core/PaintEvent
                  match event
                    (:mouse-down payload)
                      do
                        assert= 12 $ :x payload
                        assert= :select $ .unwrap-or
                          :action $ :target payload
                          , :missing
                    _ $ raise |expected-mouse-down-event
              :tags $ #{} :unit
            %{} 'TestEntry (:name |decodes-file-drop-as-fs-path)
              :code $ quote
                let
                    event $ paint-event-from-ffi
                      PaintEventFfi :file-drop $ {} (:path |/tmp/paint-demo.png) (:x 12) (:y 8)
                        :modifiers $ {} (:shift? false) (:control? false) (:alt? false) (:super? false)
                  assert-type event 'calcit-paint.core/PaintEvent
                  match event
                    (:file-drop payload)
                      do
                        assert= (fs:path |/tmp/paint-demo.png) (:path payload)
                        assert= 12 $ :x payload
                    _ $ raise |expected-file-drop-event
              :tags $ #{} :unit
            %{} 'TestEntry (:name |decodes-window-theme-payload)
              :code $ quote
                do $ let
                    event $ paint-event-from-ffi
                      PaintEventFfi :window-theme $ {} (:theme :dark) (:initial? false)
                  assert-type event 'calcit-paint.core/PaintEvent
                  match event
                    (:window-theme payload)
                      do
                        assert= :dark $ :theme payload
                        assert= false $ :initial? payload
                    _ $ raise |expected-window-theme-event
            %{} 'TestEntry (:name |decodes-file-dialog-result)
              :code $ quote
                let
                    event $ paint-event-from-ffi
                      PaintEventFfi :file-dialog-result $ {} (:request-id |open-image) (:operation :open) (:status :selected) (:path |/tmp/image.png)
                        :error $ %none
                  assert-type event 'calcit-paint.core/PaintEvent
                  match event
                    (:file-dialog-result payload)
                      do
                        assert= :selected $ :status payload
                        assert= (fs:path |/tmp/image.png)
                          .unwrap $ :path payload
                    _ $ raise |expected-file-dialog-result
            %{} 'TestEntry (:name |decodes-accessibility-action)
              :code $ quote
                let
                    event $ paint-event-from-ffi
                      PaintEventFfi :accessibility-action $ {} (:id |field-a) (:operation :focus)
                        :target $ {} (:action :focus-demo)
                  assert-type event 'calcit-paint.core/PaintEvent
                  match event
                    (:accessibility-action payload)
                      do
                        assert= |field-a $ :id payload
                        assert= :focus $ :operation payload
                        assert= :focus-demo $ .unwrap-or
                          :action $ :target payload
                          , :missing
                    _ $ raise |expected-accessibility-action
              :tags $ #{} :unit
        'push-drawing-data! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn push-drawing-data! (op data)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |push_drawing_data op data
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'String 'T
              :generics $ [] 'T
        'read-clipboard-text! $ %{} 'CodeEntry (:doc "|Read UTF-8 text from the serialized system clipboard. / 从串行系统剪贴板读取 UTF-8 文本。")
          :code $ quote
            defn read-clipboard-text! () $ &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |read_clipboard_text
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'String)
              :args $ []
        'render-to-png! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn render-to-png! (options)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |render_to_png options
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'T
              :generics $ [] 'T
        'request-frame! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn request-frame! ()
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |request_frame
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'request-window-size! $ %{} 'CodeEntry (:doc "|Queue a positive finite logical-size request for the active Paint window.")
          :code $ quote
            defn request-window-size! (width height)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |request_window_size width height
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'Number 'Number
        'save-file-dialog! $ %{} 'CodeEntry (:doc "|Queue one native save-file dialog; its result is delivered later as :file-dialog-result. / 请求一个原生保存文件对话框；结果稍后以 :file-dialog-result 送达。")
          :code $ quote
            defn save-file-dialog! (options)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |save_file_dialog options
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/PaintFileDialogOptions
        'set-window-title! $ %{} 'CodeEntry (:doc "|Queue a title update for serialized application on the active event loop.")
          :code $ quote
            defn set-window-title! (title)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |set_window_title title
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'String
        'validate-scene $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn validate-scene (scene)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |validate_scene scene
          :examples $ []
          :schema $ :: 'Fn
            {}
              :args $ [] 'T
              :generics $ [] 'T
              :return $ :: 'List 'String
        'write-clipboard-text! $ %{} 'CodeEntry (:doc "|Write UTF-8 text to the serialized system clipboard. / 向串行系统剪贴板写入 UTF-8 文本。")
          :code $ quote
            defn write-clipboard-text! (text)
              &call-dylib-edn (get-dylib-path |/dylibs/libcalcit_paint) |write_clipboard_text text
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'String
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.core $ :require
            calcit-paint.$meta :refer $ calcit-dirname
            calcit-paint.util :refer $ get-dylib-path
    'calcit-paint.main $ %{} 'FileEntry
      :defs $ {}
        '*animation-active? $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *animation-active? false)
          :examples $ []
          :schema $ :: 'Ref 'Bool
        '*animation-time-ms $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *animation-time-ms 0)
          :examples $ []
          :schema $ :: 'Ref 'Number
        '*clipboard-status $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *clipboard-status |clipboard:idle)
          :examples $ []
          :schema $ :: 'Ref 'String
        '*file-drop-status $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *file-drop-status |file-drop:idle)
          :examples $ []
          :schema $ :: 'Ref 'String
        '*pointer-dirty? $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *pointer-dirty? false)
          :examples $ []
          :schema $ :: 'Ref 'Bool
        '*pointer-status $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *pointer-status |hover:idle)
          :examples $ []
          :schema $ :: 'Ref 'String
        '*system-theme $ %{} 'CodeEntry (:doc |)
          :code $ quote (defatom *system-theme :unknown)
          :examples $ []
          :schema $ :: 'Ref 'Tag
        'export-offscreen-demo! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn export-offscreen-demo! ()
              render-to-png! $ {} (:path |offscreen-demo.png) (:width 360) (:height 180)
                :background $ [] 225 25 12
                :scene $ {} (:type :group)
                  :children $ []
                    {} (:type :cached-group) (:cache-key |offscreen-card) (:revision 1)
                      :position $ [] 20 20
                      :width 320
                      :height 140
                      :children $ []
                        {} (:type :rounded-rect)
                          :position $ [] 0 0
                          :width 320
                          :height 140
                          :radius 18
                          :fill-color $ [] 205 70 42
                        {} (:type :circle)
                          :position $ [] 75 70
                          :radius 42
                          :fill-color $ [] 38 90 58
                        {} (:type :text) (:text "|Offscreen · 离屏快照")
                          :position $ [] 140 70
                          :color $ [] 0 0 98
                          :size 22
                          :baseline :middle
                          :align :left
              println "|Exported offscreen-demo.png / 已导出离屏快照"
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'handle-file-dialog-event! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn handle-file-dialog-event! (payload)
              hint-fn $ {}
                :args $ [] 'calcit-paint.core/PaintFileDialogEvent
                :return 'Unit
              reset! *file-drop-status $ str-spaced :file-dialog (:operation payload) (:status payload) (:request-id payload) (:path payload) (:error payload)
              reset! *pointer-dirty? true
              request-frame!
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/PaintFileDialogEvent
        'handle-file-event! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn handle-file-event! (kind payload)
              reset! *file-drop-status $ str-spaced kind
                :value $ :path payload
                , |@ (:x payload) (:y payload)
              reset! *pointer-dirty? true
              request-frame!
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'Tag 'calcit-paint.core/PaintFileEvent
        'handle-file-hover-cancel! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn handle-file-hover-cancel! (payload)
              reset! *file-drop-status $ str-spaced :file-hover-cancel |@ (:x payload) (:y payload)
              reset! *pointer-dirty? true
              request-frame!
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/PaintFileHoverCancelEvent
        'handle-paint-event! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn handle-paint-event! (event)
              match event
                (:ready) (request-frame!)
                (:frame payload)
                  do
                    reset! *animation-time-ms $ :timestamp-ms payload
                    if @*animation-active?
                      do (reset! *pointer-dirty? false) (render! false) (request-frame!)
                      if @*pointer-dirty? $ do (reset! *pointer-dirty? false) (render! false)
                (:mouse-down payload)
                  handle-target-event! :mouse-down (:target payload)
                    .unwrap-or (:captured? payload) false
                (:mouse-up payload)
                  handle-target-event! :mouse-up (:target payload)
                    .unwrap-or (:captured? payload) false
                (:mouse-move payload)
                  handle-target-event! :mouse-move (:target payload)
                    .unwrap-or (:captured? payload) false
                (:mouse-leave payload)
                  handle-target-event! :mouse-leave (:target payload)
                    .unwrap-or (:captured? payload) false
                (:mouse-wheel payload)
                  handle-target-event! :mouse-wheel (:target payload)
                    .unwrap-or (:captured? payload) false
                (:pointer-enter payload)
                  handle-target-event! :pointer-enter (:target payload)
                    .unwrap-or (:captured? payload) false
                (:pointer-leave payload)
                  handle-target-event! :pointer-leave (:target payload)
                    .unwrap-or (:captured? payload) false
                (:pointer-cancel payload)
                  handle-target-event! :pointer-cancel (:target payload)
                    .unwrap-or (:captured? payload) false
                (:key-down payload)
                  handle-target-event! :key-down (:target payload) false
                (:key-up payload)
                  handle-target-event! :key-up (:target payload) false
                (:focus-in payload)
                  handle-target-event! :focus-in (:target payload) false
                (:focus-out payload)
                  handle-target-event! :focus-out (:target payload) false
                (:ime-enabled payload)
                  handle-target-event! :ime-enabled (:target payload) false
                (:ime-disabled payload)
                  handle-target-event! :ime-disabled (:target payload) false
                (:composition-start payload)
                  handle-target-event! :composition-start (:target payload) false
                (:composition-update payload)
                  handle-target-event! :composition-update (:target payload) false
                (:composition-end payload)
                  handle-target-event! :composition-end (:target payload) false
                (:text-input payload)
                  handle-target-event! :text-input (:target payload) false
                (:file-hover payload) (handle-file-event! :file-hover payload)
                (:file-drop payload) (handle-file-event! :file-drop payload)
                (:file-hover-cancel payload) (handle-file-hover-cancel! payload)
                (:file-dialog-result payload) (handle-file-dialog-event! payload)
                (:accessibility-action payload)
                  do
                    handle-target-event! (:operation payload) (:target payload) false
                    println $ str "|accessibility " (:operation payload) "|: " (:id payload)
                (:window-focus) (println |window-focus)
                (:window-blur) (println |window-blur)
                (:resize payload) (println |resize: payload)
                (:scale-factor payload) (println |scale-factor: payload)
                (:window-theme payload)
                  do
                    reset! *system-theme $ :theme payload
                    reset! *pointer-dirty? true
                    request-frame!
                (:window-title-applied payload) (println |window-title-applied: payload)
                (:window-size-request payload) (println |window-size-request: payload)
                (:window-close payload) (println |window-close: payload)
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'calcit-paint.core/PaintEvent
        'handle-target-event! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn handle-target-event! (kind target captured?)
              case-default
                .unwrap-or (:action target) :none
                println |event: kind target
                :focus-first $ focus! |field-a
                :export-snapshot $ export-offscreen-demo!
                :clipboard-copy $ do (write-clipboard-text! "|Calcit Paint clipboard / 剪贴板") (reset! *clipboard-status "|copied: Calcit Paint clipboard / 已复制") (reset! *pointer-dirty? true) (request-frame!)
                :clipboard-paste $ let
                    text $ read-clipboard-text!
                  reset! *clipboard-status $ str "|pasted: " text
                  reset! *pointer-dirty? true
                  request-frame!
                :toggle-animation $ toggle-animation!
                :window-title $ do (set-window-title! "|Calcit Paint · open dialog / 打开文件")
                  open-file-dialog! $ PaintFileDialogOptions :request-id |demo-open :title (%some "|Open image / 打开图片") :directory
                    %some $ fs:path |.
                    , :file-name (%none) :filters
                      [] $ PaintFileDialogFilter :name |Images :extensions ([] |png |jpg |jpeg)
                  reset! *file-drop-status "|file-dialog:open-pending (T)"
                  request-frame!
                :window-size $ do (request-window-size! 980 700)
                  save-file-dialog! $ PaintFileDialogOptions :request-id |demo-save :title (%some "|Save image / 保存图片") :directory
                    %some $ fs:path |.
                    , :file-name (%some |calcit-paint-export.png) :filters
                      [] $ PaintFileDialogFilter :name |PNG :extensions ([] |png)
                  reset! *file-drop-status "|file-dialog:save-pending (S)"
                  request-frame!
                :window-close $ close-window!
                :input-demo $ do
                  reset! *pointer-status $ str-spaced kind
                    .unwrap-or (:path target) ([])
                    , |captured? captured?
                  reset! *pointer-dirty? true
                  request-frame!
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'Tag 'calcit-paint.core/PaintTarget 'Bool
        'main! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn main! () (println |started)
              println $ measure-text!
                {} (:text "|Text layout / 文本排版") (:size 24) (:font-family |monospace) (:weight 700) (:style :italic) (:baseline :middle)
              println $ measure-paragraph!
                {} (:text "|Paragraph measurement / 段落测量") (:max-width 260) (:size 20) (:line-height 28) (:max-lines 2) (:ellipsis "|…")
              validate-scene-demo!
              render! true
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'reload! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn reload! () (render! false) (println "|reloads 19")
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'render! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn render! (start-loop?)
              push-drawing-data! |reset-canvas! $ if (= @*system-theme :dark) ([] 225 24 12)
                if (= @*system-theme :light) ([] 200 50 30) ([] 210 32 18)
              push-drawing-data! |render-canvas! $ {} (:type :group)
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
                  {} (:type :clip-rounded-rect)
                    :position $ [] 220 80
                    :width 260
                    :height 160
                    :radius 24
                    :children $ []
                      {} (:type :rounded-rect)
                        :position $ [] 220 80
                        :width 260
                        :height 160
                        :radius 24
                        :fill-color $ [] 220 32 22
                        :line-color $ [] 220 70 62
                        :line-width 3
                      {} (:type :opacity) (:alpha 0.85)
                        :children $ []
                          {} (:type :rounded-rect)
                            :position $ [] 230 90
                            :width 120
                            :height 70
                            :radius 16
                            :fill-color $ [] 45 80 60
                          {} (:type :ellipse)
                            :position $ [] 400 125
                            :radius-x 48
                            :radius-y 30
                            :line-color $ [] 200 80 80
                            :line-width 4
                          {} (:type :arc)
                            :position $ [] 350 205
                            :radius-x 70
                            :radius-y 32
                            :start-angle 190
                            :sweep-angle 160
                            :line-color $ [] 120 80 70
                            :line-width 5
                      {} (:type :translate) (:x 30) (:y 0)
                        :children $ []
                          {} (:type :touch-area) (:dx 65) (:dy 18)
                            :position $ [] 415 210
                            :action :input-demo
                            :path $ [] :demo :clip
                            :data :clipped-target
                            :cursor :pointer
                            :fill-color $ [] 285 74 54
                            :line-color $ [] 285 92 82
                            :line-width 2
                            :accessibility $ {} (:id |clipped-input-button) (:role :button) (:label "|Clipped input demo") (:enabled? true)
                          {} (:type :text) (:text "|Clipped hit area →")
                            :position $ [] 415 210
                            :color $ [] 0 0 98
                            :size 13
                            :baseline :middle
                            :align :center
                  {} (:type :text)
                    :text $ str "|Demo · system theme / 系统主题: " @*system-theme
                    :position $ [] 200 40
                    :color $ if (= @*system-theme :dark) ([] 48 90 72)
                      if (= @*system-theme :light) ([] 0 80 100) ([] 42 80 84)
                    :size 28
                    :weight |500
                    :align :center
                  {} (:type :group)
                    :children $ []
                      {} (:type :text) (:text "|Bold italic · top")
                        :position $ [] 530 110
                        :color $ [] 42 90 92
                        :size 24
                        :font-family |monospace
                        :weight 700
                        :style :italic
                        :baseline :top
                        :align :left
                      {} (:type :text) (:text "|Regular · middle")
                        :position $ [] 530 158
                        :color $ [] 170 76 96
                        :size 24
                        :font-family |monospace
                        :weight 400
                        :baseline :middle
                        :align :left
                      {} (:type :text) (:text "|Light · bottom")
                        :position $ [] 530 206
                        :color $ [] 42 90 88
                        :size 24
                        :font-family |monospace
                        :weight 300
                        :baseline :bottom
                        :align :left
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
                  {} (:type :touch-area) (:radius 10)
                    :position $ [] 200 200
                    :fill-color $ [] 40 80 80
                  {} (:type :key-listener) (:key |D) (:action :keyboard)
                    :path $ [] :k
                    :data :data
                  {} (:type :ops)
                    :path $ []
                      [] :move-to $ [] 200 300
                      [] :line-to $ [] 240 340
                      [] :bezier3-to ([] 400 260) ([] 200 400) ([] 300 400)
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
                      {} (:type :touch-area) (:radius 10)
                        :position $ [] 200 200
                        :fill-color $ [] 40 80 80
                  {} (:type :group)
                    :children $ []
                      {} (:type :rounded-rect)
                        :position $ [] 500 410
                        :width 180
                        :height 64
                        :radius 14
                        :fill $ {} (:type :linear-gradient)
                          :from $ [] 500 410
                          :to $ [] 680 474
                          :stops $ []
                            [] 0 $ [] 16 90 60
                            [] 0.5 $ [] 330 85 62
                            [] 1 $ [] 210 85 55
                      {} (:type :circle)
                        :position $ [] 790 442
                        :radius 52
                        :fill $ {} (:type :radial-gradient)
                          :center $ [] 772 424
                          :radius 70
                          :stops $ []
                            [] 0 $ [] 52 95 72
                            [] 0.55 $ [] 18 90 58
                            [] 1 $ [] 348 85 42
                      {} (:type :rectangle)
                        :position $ [] 500 510
                        :width 180
                        :height 58
                        :stroke $ {}
                          :paint $ {} (:type :solid)
                            :color $ [] 192 90 68
                          :width 5
                          :cap :round
                          :join :miter
                          :miter-limit 6
                          :dash $ [] 14 8
                          :dash-offset 3
                      {} (:type :rectangle)
                        :position $ [] 750 510
                        :width 140
                        :height 70
                        :fill-color $ [] 48 90 62
                      {} (:type :blend) (:mode :multiply)
                        :children $ []
                          {} (:type :circle)
                            :position $ [] 830 545
                            :radius 42
                            :fill-color $ [] 215 90 60
                  {} (:type :group)
                    :children $ []
                      {} (:type :touch-area) (:dx 150) (:dy 42)
                        :position $ [] 780 320
                        :action :input-demo
                        :path $ [] :demo :pointer :base
                        :data :capture-base
                        :cursor :grab
                        :fill-color $ [] 185 70 50
                        :line-color $ [] 185 90 85
                        :line-width 2
                      {} (:type :touch-area) (:dx 55) (:dy 28)
                        :position $ [] 855 320
                        :action :input-demo
                        :path $ [] :demo :pointer :overlay
                        :data :hover-overlay
                        :cursor :crosshair
                        :fill-color $ [] 42 82 58
                        :line-color $ [] 42 94 84
                        :line-width 2
                      {} (:type :text) (:text "|Hover + capture demo / 悬停与捕获")
                        :position $ [] 760 306
                        :color $ [] 0 0 100
                        :size 17
                        :align :center
                      {} (:type :text) (:text "|Drag outside a region; overlap uses crosshair")
                        :position $ [] 760 332
                        :color $ [] 0 0 94
                        :size 12
                        :align :center
                      {} (:type :text) (:text @*pointer-status)
                        :position $ [] 780 370
                        :color $ [] 45 18 96
                        :size 13
                        :align :center
                      {} (:type :key-listener) (:key |I) (:action :input-demo)
                        :path $ [] :demo :keyboard
                        :data :keyboard-demo
                  {} (:type :group)
                    :children $ []
                      {} (:type :paragraph) (:text "|Calcit Paint paragraph\n中文段落 · explicit newline")
                        :position $ [] 40 610
                        :max-width 300
                        :color $ [] 42 90 92
                        :size 20
                        :line-height 28
                        :align :left
                      {} (:type :paragraph) (:text "|A constrained paragraph demonstrates Unicode-safe wrapping and ellipsis. 受限宽度段落展示安全换行与省略号。")
                        :position $ [] 380 610
                        :max-width 320
                        :color $ [] 170 76 96
                        :size 18
                        :line-height 26
                        :max-lines 2
                        :ellipsis "|…"
                        :align :center
                      {} (:type :paragraph) (:text "|مرحبا بالعالم · تخطيط النص من اليمين إلى اليسار")
                        :position $ [] 740 610
                        :max-width 320
                        :color $ [] 200 82 90
                        :size 20
                        :line-height 30
                        :max-lines 2
                        :ellipsis "|…"
                        :direction :rtl
                        :align :right
                  {} (:type :group)
                    :children $ []
                      {} (:type :focus-area) (:focus-id |field-a) (:tab-index 0) (:text-input? true)
                        :position $ [] 180 450
                        :dx 140
                        :dy 32
                        :action :focus-demo
                        :path $ [] :field-a
                        :data :ime-field
                        :fill-color $ [] 215 70 45
                        :line-color $ [] 215 88 76
                        :line-width 3
                        :accessibility $ {} (:id |field-a) (:role :text-input) (:label "|Focus A IME input") (:value |) (:enabled? true) (:focusable? true)
                      {} (:type :text) (:text "|Focus A · IME text input")
                        :position $ [] 180 450
                        :color $ [] 0 0 98
                        :size 18
                        :baseline :middle
                        :align :center
                      {} (:type :focus-area) (:focus-id |field-b) (:tab-index 1) (:text-input? true)
                        :position $ [] 180 525
                        :dx 140
                        :dy 32
                        :action :focus-demo
                        :path $ [] :field-b
                        :data :ime-field
                        :fill-color $ [] 165 65 42
                        :line-color $ [] 165 88 74
                        :line-width 3
                        :accessibility $ {} (:id |field-b) (:role :text-input) (:label "|Focus B Tab input") (:value |) (:enabled? true) (:focusable? true)
                      {} (:type :text) (:text "|Focus B · Tab / Shift+Tab")
                        :position $ [] 180 525
                        :color $ [] 0 0 98
                        :size 18
                        :baseline :middle
                        :align :center
                      {} (:type :key-listener) (:key |K) (:action :focus-first)
                        :modifiers $ {} (:shift? true)
                        :data :shortcut-demo
                      {} (:type :key-listener) (:key |P) (:action :export-snapshot)
                        :modifiers $ {} (:shift? true)
                        :data :offscreen-demo
                      {} (:type :key-listener) (:key |Enter) (:focus-id |field-a) (:action :field-submit) (:data :focus-scoped-key)
                      {} (:type :text) (:text "|Click, Tab, IME; Shift+K focuses A; Shift+P exports PNG")
                        :position $ [] 180 580
                        :color $ [] 45 18 96
                        :size 14
                        :align :center
                  {} (:type :group)
                    :children $ []
                      {} (:type :rectangle)
                        :position $ [] 310 12
                        :width 100
                        :height 58
                        :fill-color $ [] 220 16 24
                      {} (:type :image) (:file-path |resources/calcit.png) (:x 310) (:y 12) (:w 100) (:h 58)
                      {} (:type :text) (:text |fill/nearest)
                        :position $ [] 360 78
                        :color $ [] 0 0 96
                        :size 11
                        :align :center
                      {} (:type :rectangle)
                        :position $ [] 425 12
                        :width 100
                        :height 58
                        :fill-color $ [] 220 16 24
                      {} (:type :image) (:file-path |resources/calcit.png) (:x 425) (:y 12) (:w 100) (:h 58) (:fit :contain) (:sampling :linear)
                      {} (:type :text) (:text |contain/linear)
                        :position $ [] 475 78
                        :color $ [] 0 0 96
                        :size 11
                        :align :center
                      {} (:type :rectangle)
                        :position $ [] 540 12
                        :width 100
                        :height 58
                        :fill-color $ [] 220 16 24
                      {} (:type :image) (:file-path |resources/calcit.png) (:x 540) (:y 12) (:w 100) (:h 58)
                        :crop $ {} (:x 80) (:y 80) (:w 320) (:h 320)
                        :fit :cover
                        :sampling :cubic
                      {} (:type :text) (:text |cover/cubic)
                        :position $ [] 590 78
                        :color $ [] 0 0 96
                        :size 11
                        :align :center
                  {} (:type :group)
                    :children $ []
                      {} (:type :circle) (:radius 22)
                        :position $ []
                          + 900 $ * 55
                            sin $ / @*animation-time-ms 420
                          , 270
                        :fill-color $ [] 285 82 62
                        :line-color $ [] 285 95 86
                        :line-width 3
                      {} (:type :text)
                        :text $ if @*animation-active? "|A: pause animation / 暂停动画" "|A: start animation / 开始动画"
                        :position $ [] 900 315
                        :color $ [] 285 72 92
                        :size 15
                        :align :center
                      {} (:type :key-listener) (:key |A) (:action :toggle-animation)
                  {} (:type :group)
                    :children $ []
                      {} (:type :text) (:text "|T title · S resize · Q close / 标题 · 尺寸 · 关闭")
                        :position $ [] 900 215
                        :color $ [] 195 62 92
                        :size 13
                        :align :center
                      {} (:type :text) (:text @*file-drop-status)
                        :position $ [] 900 240
                        :color $ [] 195 62 92
                        :size 12
                        :align :center
                      {} (:type :group)
                        :children $ []
                          {} (:type :text) (:text "|Shift+C copy · Shift+V paste / 复制 · 粘贴")
                            :position $ [] 900 265
                            :color $ [] 195 62 92
                            :size 12
                            :align :center
                          {} (:type :text) (:text @*clipboard-status)
                            :position $ [] 900 285
                            :color $ [] 195 62 92
                            :size 11
                            :align :center
                          {} (:type :key-listener) (:key |C) (:action :clipboard-copy)
                            :modifiers $ {} (:shift? true)
                          {} (:type :key-listener) (:key |V) (:action :clipboard-paste)
                            :modifiers $ {} (:shift? true)
                      {} (:type :key-listener) (:key |T) (:action :window-title)
                      {} (:type :key-listener) (:key |S) (:action :window-size)
                      {} (:type :key-listener) (:key |Q) (:action :window-close)
                  {} (:type :cached-group) (:cache-key |window-static-badge) (:revision 1)
                    :position $ [] 900 80
                    :width 170
                    :height 110
                    :children $ []
                      {} (:type :rounded-rect)
                        :position $ [] 0 0
                        :width 170
                        :height 110
                        :radius 16
                        :fill-color $ [] 225 55 28
                        :line-color $ [] 195 82 68
                        :line-width 3
                      {} (:type :circle)
                        :position $ [] 42 55
                        :radius 25
                        :fill-color $ [] 42 90 58
                      {} (:type :text) (:text |cached)
                        :position $ [] 80 48
                        :color $ [] 0 0 96
                        :size 18
                        :align :left
                      {} (:type :text) (:text "|Shift+P → PNG")
                        :position $ [] 80 72
                        :color $ [] 0 0 86
                        :size 13
                        :align :left
              if start-loop?
                launch-canvas-typed! (WindowOptions :title "|Calcit Paint · typed events / 强类型事件" :width 1100 :height 760 :min-width 720 :min-height 520 :resizable? true)
                  fn (event) (handle-paint-event! event)
                , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ [] 'Bool
        'toggle-animation! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn toggle-animation! ()
              if @*animation-active?
                do (reset! *animation-active? false) (render! false)
                do (reset! *animation-active? true) (request-frame!)
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
        'validate-scene-demo! $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn validate-scene-demo! () $ let
                valid-scene $ {} (:type :group)
                  :children $ []
                    {} (:type :rounded-rect) (:width 160) (:height 70) (:radius 12)
                valid-diagnostics $ validate-scene valid-scene
                invalid-diagnostics $ validate-scene
                  {} (:type :group)
                    :children $ []
                      {} $ :type :unknown-demo-shape
                      {} (:type :group)
                        :children $ [] true
              if (empty? valid-diagnostics) (println "|scene validation passed / 场景校验通过")
                raise $ str "|unexpected diagnostics for valid scene: " valid-diagnostics
              if
                = 2 $ count invalid-diagnostics
                println "|expected validation diagnostics / 预期校验诊断: " invalid-diagnostics
                raise $ str "|expected two invalid-scene diagnostics, got: " invalid-diagnostics
              , &unit
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'Unit)
              :args $ []
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.main $ :require
            calcit-paint.core :refer $ WindowOptions PaintEvent PaintTarget PaintFileDialogOptions PaintFileDialogFilter PaintFileDialogEvent launch-canvas-typed! push-drawing-data! measure-text! measure-paragraph! focus! render-to-png! validate-scene request-frame! set-window-title! request-window-size! close-window! read-clipboard-text! write-clipboard-text! open-file-dialog! save-file-dialog!
    'calcit-paint.util $ %{} 'FileEntry
      :defs $ {}
        'get-dylib-ext $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defmacro get-dylib-ext () $ case-default (&get-os) |.so (:macos |.dylib) (:windows |.dll)
          :examples $ []
          :schema $ :: 'Macro
            {}
              :capabilities $ #{} :platform-read
              :expansion $ :: 'Expr 'String
              :required $ []
        'get-dylib-path $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn get-dylib-path (p)
              str (or-current-path calcit-dirname) p $ get-dylib-ext
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'String)
              :args $ [] 'String
        'or-current-path $ %{} 'CodeEntry (:doc |)
          :code $ quote
            defn or-current-path (p)
              if (blank? p) |. p
          :examples $ []
          :schema $ :: 'Fn
            {} (:return 'String)
              :args $ [] 'String
      :ns $ %{} 'NsEntry (:doc |)
        :code $ quote
          ns calcit-paint.util $ :require
            calcit-paint.$meta :refer $ calcit-dirname calcit-filename
