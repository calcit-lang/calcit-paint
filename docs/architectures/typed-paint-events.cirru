{}
  :schema-version 1
  :feature 'typed-paint-events
  :doc "|Provide an additive nominal PaintEvent protocol while preserving the legacy map callback APIs."
  :roots $ #{} 'calcit-paint.core/PaintEvent 'calcit-paint.core/launch-canvas-typed!
  :definitions $ {}
    'calcit-paint.core/WindowOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Existing strict startup configuration reused by the typed launch API."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct WindowOptions (:title 'String) (:width 'Number) (:height 'Number) (:min-width 'Number) (:min-height 'Number) (:resizable? 'Bool)
    'calcit-paint.core/PaintModifiers $ {}
      :mode :ensure
      :kind :data
      :doc "|Closed keyboard and pointer modifier state."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintModifiers (:shift? 'Bool) (:control? 'Bool) (:alt? 'Bool) (:super? 'Bool)
    'calcit-paint.core/PaintTarget $ {}
      :mode :ensure
      :kind :data
      :doc "|Application-defined target values kept at one explicit open boundary."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintTarget (:action (:: 'Option 'Dynamic)) (:path (:: 'Option 'Dynamic)) (:data (:: 'Option 'Dynamic))
    'calcit-paint.core/PaintPointerEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed pointer, mouse, hover, cancellation, and wheel payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintPointerEvent (:x 'Number) (:y 'Number) (:clicks 'Number) (:modifiers 'calcit-paint.core/PaintModifiers) (:target 'calcit-paint.core/PaintTarget) (:dx (:: 'Option 'Number)) (:dy (:: 'Option 'Number)) (:button (:: 'Option 'Tag)) (:button-id (:: 'Option 'Number)) (:cursor (:: 'Option 'Tag)) (:captured? (:: 'Option 'Bool)) (:cancelled? (:: 'Option 'Bool)) (:reason (:: 'Option 'Tag)) (:unit (:: 'Option 'Tag))
    'calcit-paint.core/PaintKeyboardEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed logical and physical keyboard payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintKeyboardEvent (:key-code 'Number) (:physical-key 'String) (:name 'String) (:modifiers 'calcit-paint.core/PaintModifiers) (:focus-id (:: 'Option 'String)) (:shortcut? (:: 'Option 'Bool)) (:target 'calcit-paint.core/PaintTarget)
    'calcit-paint.core/PaintFocusEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed focus transition payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFocusEvent (:focus-id 'String) (:related-focus-id (:: 'Option 'String)) (:reason 'Tag) (:target 'calcit-paint.core/PaintTarget)
    'calcit-paint.core/PaintTextInputEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed IME composition and committed text payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintTextInputEvent (:focus-id 'String) (:text 'String) (:cursor-start (:: 'Option 'Number)) (:cursor-end (:: 'Option 'Number)) (:cancelled? (:: 'Option 'Bool)) (:target 'calcit-paint.core/PaintTarget)
    'calcit-paint.core/PaintFrameEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed on-demand animation frame payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintFrameEvent (:frame 'Number) (:timestamp-ms 'Number) (:delta-ms 'Number) (:width 'Number) (:height 'Number) (:scale-factor 'Number)
    'calcit-paint.core/PaintWindowMetricsEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed logical window metrics and scale payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintWindowMetricsEvent (:width 'Number) (:height 'Number) (:scale-factor 'Number)
    'calcit-paint.core/PaintWindowTitleEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed applied window-title acknowledgement."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintWindowTitleEvent (:title 'String)
    'calcit-paint.core/PaintWindowSizeEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed logical-size request acknowledgement."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintWindowSizeEvent (:status 'Tag) (:requested-width 'Number) (:requested-height 'Number) (:actual-width (:: 'Option 'Number)) (:actual-height (:: 'Option 'Number)) (:matched? (:: 'Option 'Bool)) (:scale-factor 'Number)
    'calcit-paint.core/PaintWindowCloseEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed exactly-once window close payload."
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintWindowCloseEvent (:reason 'Tag)
    'calcit-paint.core/PaintEventFfi $ {}
      :mode :ensure
      :kind :data
      :doc "|Internal generic envelope received from the native typed-event transport."
      :schema $ :: 'EnumDef
      :code $ quote
        defenum PaintEventFfi ([] 'Raw) (:ready) (:frame 'Raw) (:mouse-down 'Raw) (:mouse-up 'Raw) (:mouse-move 'Raw) (:mouse-leave 'Raw) (:mouse-wheel 'Raw) (:pointer-enter 'Raw) (:pointer-leave 'Raw) (:pointer-cancel 'Raw) (:key-down 'Raw) (:key-up 'Raw) (:focus-in 'Raw) (:focus-out 'Raw) (:ime-enabled 'Raw) (:ime-disabled 'Raw) (:composition-start 'Raw) (:composition-update 'Raw) (:composition-end 'Raw) (:text-input 'Raw) (:window-focus) (:window-blur) (:resize 'Raw) (:scale-factor 'Raw) (:window-title-applied 'Raw) (:window-size-request 'Raw) (:window-close 'Raw)
    'calcit-paint.core/PaintEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Nominal exhaustive event protocol for typed Paint callbacks."
      :schema $ :: 'EnumDef
      :code $ quote
        defenum PaintEvent (:ready) (:frame 'calcit-paint.core/PaintFrameEvent) (:mouse-down 'calcit-paint.core/PaintPointerEvent) (:mouse-up 'calcit-paint.core/PaintPointerEvent) (:mouse-move 'calcit-paint.core/PaintPointerEvent) (:mouse-leave 'calcit-paint.core/PaintPointerEvent) (:mouse-wheel 'calcit-paint.core/PaintPointerEvent) (:pointer-enter 'calcit-paint.core/PaintPointerEvent) (:pointer-leave 'calcit-paint.core/PaintPointerEvent) (:pointer-cancel 'calcit-paint.core/PaintPointerEvent) (:key-down 'calcit-paint.core/PaintKeyboardEvent) (:key-up 'calcit-paint.core/PaintKeyboardEvent) (:focus-in 'calcit-paint.core/PaintFocusEvent) (:focus-out 'calcit-paint.core/PaintFocusEvent) (:ime-enabled 'calcit-paint.core/PaintTextInputEvent) (:ime-disabled 'calcit-paint.core/PaintTextInputEvent) (:composition-start 'calcit-paint.core/PaintTextInputEvent) (:composition-update 'calcit-paint.core/PaintTextInputEvent) (:composition-end 'calcit-paint.core/PaintTextInputEvent) (:text-input 'calcit-paint.core/PaintTextInputEvent) (:window-focus) (:window-blur) (:resize 'calcit-paint.core/PaintWindowMetricsEvent) (:scale-factor 'calcit-paint.core/PaintWindowMetricsEvent) (:window-title-applied 'calcit-paint.core/PaintWindowTitleEvent) (:window-size-request 'calcit-paint.core/PaintWindowSizeEvent) (:window-close 'calcit-paint.core/PaintWindowCloseEvent)
    'calcit-paint.core/paint-event-from-ffi $ {}
      :mode :ensure
      :kind :fn
      :doc "|Strictly decode one native typed-event envelope into the public nominal protocol."
      :params $ [] 'event
      :schema $ :: 'Fn
        {}
          :args $ []
            :: 'calcit-paint.core/PaintEventFfi
              :: 'Map 'Tag 'Dynamic
          :return 'calcit-paint.core/PaintEvent
    'calcit-paint.core/launch-canvas-typed! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Launch the configured single Paint window with a nominal PaintEvent callback."
      :params $ [] 'options 'cb
      :schema $ :: 'Fn
        {}
          :args $ [] 'calcit-paint.core/WindowOptions
            :: 'Fn $ {} (:return 'R)
              :args $ [] 'calcit-paint.core/PaintEvent
          :return 'Unit
          :generics $ [] 'R
  :edges $ #{}
    :: :call 'calcit-paint.core/launch-canvas-typed! 'calcit-paint.core/paint-event-from-ffi
    :: :type 'calcit-paint.core/launch-canvas-typed! 'calcit-paint.core/WindowOptions
    :: :type 'calcit-paint.core/launch-canvas-typed! 'calcit-paint.core/PaintEvent
    :: :type 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.core/PaintEventFfi
    :: :type 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.core/PaintEvent
