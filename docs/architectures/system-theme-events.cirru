{}
  :schema-version 1
  :feature 'system-theme-events
  :doc "|Expose initial and runtime system theme changes through the typed Paint event protocol. / 通过强类型 Paint 事件协议暴露初始与运行期系统主题变化。"
  :roots $ #{} 'calcit-paint.core/PaintEvent 'calcit-paint.core/paint-event-from-ffi
  :definitions $ {}
    'calcit-paint.core/PaintWindowThemeEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed system theme payload: :light, :dark, or :unknown, plus whether it is the initial observation. / 强类型系统主题 payload：:light、:dark 或 :unknown，并标明是否为初始观测。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintWindowThemeEvent (:theme 'Tag) (:initial? 'Bool)
    'calcit-paint.core/PaintEventFfi $ {}
      :mode :ensure
      :kind :data
      :doc "|Internal typed-event transport envelope extended with :window-theme. / 扩展 :window-theme 的内部强类型事件传输 envelope。"
      :schema $ :: 'EnumDef
      :code $ quote
        defenum PaintEventFfi ([] 'Raw) (:ready) (:frame 'Raw) (:mouse-down 'Raw) (:mouse-up 'Raw) (:mouse-move 'Raw) (:mouse-leave 'Raw) (:mouse-wheel 'Raw) (:pointer-enter 'Raw) (:pointer-leave 'Raw) (:pointer-cancel 'Raw) (:key-down 'Raw) (:key-up 'Raw) (:focus-in 'Raw) (:focus-out 'Raw) (:ime-enabled 'Raw) (:ime-disabled 'Raw) (:composition-start 'Raw) (:composition-update 'Raw) (:composition-end 'Raw) (:text-input 'Raw) (:file-hover 'Raw) (:file-drop 'Raw) (:file-hover-cancel 'Raw) (:window-focus) (:window-blur) (:resize 'Raw) (:scale-factor 'Raw) (:window-theme 'Raw) (:window-title-applied 'Raw) (:window-size-request 'Raw) (:window-close 'Raw)
    'calcit-paint.core/PaintEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Public exhaustive PaintEvent protocol extended with a nominal system-theme payload. / 使用 nominal 系统主题 payload 扩展公开穷尽 PaintEvent 协议。"
      :schema $ :: 'EnumDef
      :code $ quote
        defenum PaintEvent (:ready) (:frame 'calcit-paint.core/PaintFrameEvent) (:mouse-down 'calcit-paint.core/PaintPointerEvent) (:mouse-up 'calcit-paint.core/PaintPointerEvent) (:mouse-move 'calcit-paint.core/PaintPointerEvent) (:mouse-leave 'calcit-paint.core/PaintPointerEvent) (:mouse-wheel 'calcit-paint.core/PaintPointerEvent) (:pointer-enter 'calcit-paint.core/PaintPointerEvent) (:pointer-leave 'calcit-paint.core/PaintPointerEvent) (:pointer-cancel 'calcit-paint.core/PaintPointerEvent) (:key-down 'calcit-paint.core/PaintKeyboardEvent) (:key-up 'calcit-paint.core/PaintKeyboardEvent) (:focus-in 'calcit-paint.core/PaintFocusEvent) (:focus-out 'calcit-paint.core/PaintFocusEvent) (:ime-enabled 'calcit-paint.core/PaintTextInputEvent) (:ime-disabled 'calcit-paint.core/PaintTextInputEvent) (:composition-start 'calcit-paint.core/PaintTextInputEvent) (:composition-update 'calcit-paint.core/PaintTextInputEvent) (:composition-end 'calcit-paint.core/PaintTextInputEvent) (:text-input 'calcit-paint.core/PaintTextInputEvent) (:file-hover 'calcit-paint.core/PaintFileEvent) (:file-drop 'calcit-paint.core/PaintFileEvent) (:file-hover-cancel 'calcit-paint.core/PaintFileHoverCancelEvent) (:window-focus) (:window-blur) (:resize 'calcit-paint.core/PaintWindowMetricsEvent) (:scale-factor 'calcit-paint.core/PaintWindowMetricsEvent) (:window-theme 'calcit-paint.core/PaintWindowThemeEvent) (:window-title-applied 'calcit-paint.core/PaintWindowTitleEvent) (:window-size-request 'calcit-paint.core/PaintWindowSizeEvent) (:window-close 'calcit-paint.core/PaintWindowCloseEvent)
    'calcit-paint.core/paint-event-from-ffi $ {}
      :mode :ensure
      :kind :fn
      :doc "|Strictly decode :window-theme into PaintWindowThemeEvent. / 严格将 :window-theme 解码为 PaintWindowThemeEvent。"
      :params $ [] 'event
      :schema $ :: 'Fn $ {}
        :args $ []
          :: 'calcit-paint.core/PaintEventFfi $ :: 'Map 'Tag 'Dynamic
        :return 'calcit-paint.core/PaintEvent
      :code $ quote
        defn paint-event-from-ffi (event)
          todo!
  :edges $ #{}
    :: :type 'calcit-paint.core/PaintEvent 'calcit-paint.core/PaintWindowThemeEvent
    :: :type 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.core/PaintEventFfi
    :: :type 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.core/PaintEvent
