{}
  :schema-version 1
  :feature 'accessibility-semantic-tree
  :doc "|Expose explicit scene accessibility metadata as a synchronized semantic tree and return system focus/activate requests through a closed typed event. / 将显式 scene 可访问性元数据同步为语义树，并通过封闭强类型事件回传系统 focus/activate 请求。"
  :roots $ #{} 'calcit-paint.core/PaintAccessibilityActionEvent 'calcit-paint.core/paint-event-from-ffi
  :definitions $ {}
    'calcit-paint.core/PaintAccessibilityActionEvent $ {}
      :mode :ensure
      :kind :data
      :doc "|Typed accessibility action from the platform semantic tree. Operation is :focus or :activate. / 来自平台语义树的强类型可访问性动作。operation 为 :focus 或 :activate。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintAccessibilityActionEvent (:id 'String) (:operation 'Tag) (:target 'calcit-paint.core/PaintTarget)
    'calcit-paint.core/paint-event-from-ffi $ {}
      :mode :ensure
      :kind :fn
      :params $ [] 'event
      :doc "|Strictly decode typed native Paint events, including accessibility actions. / 严格解码强类型 native Paint event，包含可访问性动作。"
      :schema $ :: 'Fn $ {}
        :args $ []
          :: 'calcit-paint.core/PaintEventFfi $ :: 'Map 'Tag 'Dynamic
        :return 'calcit-paint.core/PaintEvent
  :edges $ #{}
