{}
  :schema-version 1
  :feature 'accessibility-text-selection
  :doc "|Publish AccessKit SetTextSelection and ReplaceSelectedText as typed Calcit accessibility actions over Unicode-safe selection metadata while the Calcit app retains ownership of its state. / 通过 Unicode-safe selection metadata 将 AccessKit SetTextSelection 与 ReplaceSelectedText 发布为强类型 Calcit 无障碍动作，同时由 Calcit 应用保留状态所有权。"
  :roots $ #{} 'calcit-paint.core/PaintAccessibilityActionEvent 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.main/handle-paint-event! 'calcit-paint.main/render!
  :definitions $ {}
    'calcit-paint.core/PaintAccessibilityActionEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed accessibility request. :set-value carries value; selection operations carry Unicode scalar selection-start/selection-end; replacement carries text. / 强类型无障碍请求。:set-value 携带 value；selection 操作携带 Unicode scalar 的 selection-start/selection-end；替换携带 text。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintAccessibilityActionEvent (:id 'String) (:operation 'Tag) (:target 'calcit-paint.core/PaintTarget)
          :value $ :: 'Option 'String
          :selection-start $ :: 'Option 'Number
          :selection-end $ :: 'Option 'Number
          :text $ :: 'Option 'String
    'calcit-paint.core/paint-event-from-ffi $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'event
      :doc "|Strictly decode accessibility actions, requiring selection payloads for selection operations and text for replacement. / 严格解码无障碍动作；selection 操作要求 selection payload，替换要求 text。"
      :schema $ :: 'Fn $ {}
        :args $ []
          :: 'calcit-paint.core/PaintEventFfi $ :: 'Map 'Tag 'Dynamic
        :return 'calcit-paint.core/PaintEvent
    'calcit-paint.main/handle-paint-event! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'event
      :doc "|Apply selection, whole-value, and replacement accessibility requests to Calcit state, then redraw. / 将 selection、整值替换与文本替换请求应用到 Calcit 状态，然后重绘。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.core/PaintEvent
        :return 'Unit
    'calcit-paint.main/render! $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'start-loop?
      :doc "|Render the text-input value and its Unicode scalar selection metadata. / 渲染文本输入 value 与其 Unicode scalar selection metadata。"
      :schema $ :: 'Fn $ {}
        :args $ [] 'Bool
        :return 'Unit
  :edges $ #{}
    :: :type 'calcit-paint.core/paint-event-from-ffi 'calcit-paint.core/PaintAccessibilityActionEvent
    :: :call 'calcit-paint.main/handle-paint-event! 'calcit-paint.main/render!
