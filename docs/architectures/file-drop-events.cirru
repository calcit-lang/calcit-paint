{}
  :schema-version 1
  :feature 'file-drop-events
  :doc "|Extend compatible and nominal Paint callbacks with native file hover/drop lifecycle events."
  :roots $ #{} 'calcit-paint.core/PaintEvent 'calcit-paint.core/PaintFileEvent
  :definitions $ {}
    'calcit-paint.core/PaintModifiers $ {}
      :mode :ensure
      :kind :data
      :doc "|Existing closed keyboard and pointer modifier state reused by file events."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintModifiers (:shift? 'Bool) (:control? 'Bool) (:alt? 'Bool) (:super? 'Bool)
    'calcit-paint.core/PaintFileEventWire $ {}
      :mode :ensure
      :kind :data
      :doc "|Internal UTF-8 wire payload decoded before constructing a nominal FsPath."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileEventWire (:path 'String) (:x 'Number) (:y 'Number) (:modifiers 'calcit-paint.core/PaintModifiers)
    'calcit-paint.core/PaintFileEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed file hover/drop payload with a nominal Calcit filesystem path."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileEvent (:path 'FsPath) (:x 'Number) (:y 'Number) (:modifiers 'calcit-paint.core/PaintModifiers)
    'calcit-paint.core/PaintFileHoverCancelEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed file-hover cancellation payload without a fabricated path."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFileHoverCancelEvent (:x 'Number) (:y 'Number) (:modifiers 'calcit-paint.core/PaintModifiers)
    'calcit-paint.core/PaintEventFfi $ {}
      :mode :ensure
      :kind :data
      :doc "|Internal generic envelope received from the native typed-event transport."
      :schema $ :: 'EnumDef
      :code $ quote
        defenum PaintEventFfi ([] 'Raw) (:ready) (:frame 'Raw) (:mouse-down 'Raw) (:mouse-up 'Raw) (:mouse-move 'Raw) (:mouse-leave 'Raw) (:mouse-wheel 'Raw) (:pointer-enter 'Raw) (:pointer-leave 'Raw) (:pointer-cancel 'Raw) (:key-down 'Raw) (:key-up 'Raw) (:focus-in 'Raw) (:focus-out 'Raw) (:ime-enabled 'Raw) (:ime-disabled 'Raw) (:composition-start 'Raw) (:composition-update 'Raw) (:composition-end 'Raw) (:text-input 'Raw) (:file-hover 'Raw) (:file-drop 'Raw) (:file-hover-cancel 'Raw) (:window-focus) (:window-blur) (:resize 'Raw) (:scale-factor 'Raw) (:window-title-applied 'Raw) (:window-size-request 'Raw) (:window-close 'Raw)
    'calcit-paint.core/PaintEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Nominal exhaustive event protocol for typed Paint callbacks."
      :schema $ :: 'EnumDef
      :code $ quote
        defenum PaintEvent (:ready) (:frame 'calcit-paint.core/PaintFrameEvent) (:mouse-down 'calcit-paint.core/PaintPointerEvent) (:mouse-up 'calcit-paint.core/PaintPointerEvent) (:mouse-move 'calcit-paint.core/PaintPointerEvent) (:mouse-leave 'calcit-paint.core/PaintPointerEvent) (:mouse-wheel 'calcit-paint.core/PaintPointerEvent) (:pointer-enter 'calcit-paint.core/PaintPointerEvent) (:pointer-leave 'calcit-paint.core/PaintPointerEvent) (:pointer-cancel 'calcit-paint.core/PaintPointerEvent) (:key-down 'calcit-paint.core/PaintKeyboardEvent) (:key-up 'calcit-paint.core/PaintKeyboardEvent) (:focus-in 'calcit-paint.core/PaintFocusEvent) (:focus-out 'calcit-paint.core/PaintFocusEvent) (:ime-enabled 'calcit-paint.core/PaintTextInputEvent) (:ime-disabled 'calcit-paint.core/PaintTextInputEvent) (:composition-start 'calcit-paint.core/PaintTextInputEvent) (:composition-update 'calcit-paint.core/PaintTextInputEvent) (:composition-end 'calcit-paint.core/PaintTextInputEvent) (:text-input 'calcit-paint.core/PaintTextInputEvent) (:file-hover 'calcit-paint.core/PaintFileEvent) (:file-drop 'calcit-paint.core/PaintFileEvent) (:file-hover-cancel 'calcit-paint.core/PaintFileHoverCancelEvent) (:window-focus) (:window-blur) (:resize 'calcit-paint.core/PaintWindowMetricsEvent) (:scale-factor 'calcit-paint.core/PaintWindowMetricsEvent) (:window-title-applied 'calcit-paint.core/PaintWindowTitleEvent) (:window-size-request 'calcit-paint.core/PaintWindowSizeEvent) (:window-close 'calcit-paint.core/PaintWindowCloseEvent)
  :edges $ #{}
    :: :type 'calcit-paint.core/PaintFileEventWire 'calcit-paint.core/PaintModifiers
    :: :type 'calcit-paint.core/PaintFileEvent 'calcit-paint.core/PaintModifiers
    :: :type 'calcit-paint.core/PaintFileHoverCancelEvent 'calcit-paint.core/PaintModifiers
    :: :type 'calcit-paint.core/PaintEvent 'calcit-paint.core/PaintFileEvent
    :: :type 'calcit-paint.core/PaintEvent 'calcit-paint.core/PaintFileHoverCancelEvent
