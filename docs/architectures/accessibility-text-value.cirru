{}
  :schema-version 1
  :feature 'accessibility-text-value
  :doc "|Return platform text-value requests through the nominal Paint event protocol while the Calcit app retains ownership of its state. / 通过 nominal Paint 事件协议回传平台文本值请求，同时由 Calcit 应用保留状态所有权。"
  :roots $ #{} 'calcit-paint.core/PaintAccessibilityActionEvent 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.main/handle-paint-event! 'calcit-paint.main/render!
  :definitions $ {}
    'calcit-paint.core/PaintAccessibilityActionEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed semantic request. :set-value carries a String in value; :focus and :activate retain none. / 强类型语义请求。:set-value 在 value 中携带 String；:focus 与 :activate 保持 none。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintAccessibilityActionEvent (:id 'String) (:operation 'Tag) (:target 'calcit-paint.core/PaintTarget) (:value (:: 'Option 'String))
    'calcit-paint.core/paint-event-from-ffi $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'event
      :doc "|Strictly decode focus, activate, and set-value accessibility actions. / 严格解码 focus、activate 与 set-value 无障碍动作。"
      :schema $ :: 'Fn $ {}
        :args $ []
          :: 'calcit-paint.core/PaintEventFfi $ :: 'Map 'Tag 'Dynamic
        :return 'calcit-paint.core/PaintEvent
    'calcit-paint.main/handle-paint-event! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'event
      :doc "|Apply the demo set-value event in Calcit state, then redraw the semantic value. / 在 Calcit 状态中应用 demo set-value 事件，并重绘语义值。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.core/PaintEvent
        :return 'Unit
    'calcit-paint.main/render! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'start-loop?
      :doc "|Render the value controlled by the accessible text-input demo. / 渲染由无障碍文本输入 demo 控制的值。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'Bool
        :return 'Unit
  :edges $ #{}
    :: :type 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.core/PaintAccessibilityActionEvent
    :: :call 'calcit-paint.main/handle-paint-event! 'calcit-paint.main/render!
