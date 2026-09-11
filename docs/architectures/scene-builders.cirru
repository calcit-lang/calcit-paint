{}
  :schema-version 1
  :feature 'scene-builders
  :doc "|Provide opt-in, transparent Calcit builders for common controls that emit the existing public scene-map DSL without hiding action/path/data, focus, or accessibility behavior. / 提供 opt-in、透明的常用控件 Calcit builder，输出既有公开 scene-map DSL，且不隐藏 action/path/data、focus 或无障碍语义。"
  :roots $ #{} 'calcit-paint.ui/button 'calcit-paint.ui/touch-container 'calcit-paint.ui/text-input-focus-area 'calcit-paint.ui/icon-label 'calcit-paint.ui/demo-scene
  :definitions $ {}
    'calcit-paint.ui/ButtonOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Options for an accessible touch button: explicit id/label/geometry plus optional target, enabled state, and colors. / 可访问触摸按钮的选项：显式 id/label/geometry，以及可选 target、enabled 状态与颜色。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct ButtonOptions (:id 'String) (:label 'String) (:x 'Number) (:y 'Number) (:dx 'Number) (:dy 'Number)
          :action $ :: 'Option 'Dynamic
          :path $ :: 'Option 'Dynamic
          :data $ :: 'Option 'Dynamic
          :enabled? $ :: 'Option 'Bool
          :fill-color $ :: 'Option (:: 'List 'Number)
          :text-color $ :: 'Option (:: 'List 'Number)
          :text-size $ :: 'Option 'Number
    'calcit-paint.ui/button $ {}
      :mode :ensure
      :kind :fn
      :doc "|Build one accessible touch-area button scene map with a centered label; :action/:path/:data stay explicit and are never hidden. / 构建一个带居中标签的可访问 touch-area button scene map；:action/:path/:data 保持显式，不会被隐藏。"
      :params $ [] 'options
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.ui/ButtonOptions
        :return $ :: 'Map 'Tag 'Dynamic
    'calcit-paint.ui/TouchContainerOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Options for a labelled accessible touch container, including an optional :role, stroke, and nested children. / 带标签的可访问 touch 容器选项，包含可选 :role、描边与嵌套 children。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct TouchContainerOptions (:id 'String) (:label 'String) (:role 'Tag) (:x 'Number) (:y 'Number) (:dx 'Number) (:dy 'Number)
          :action $ :: 'Option 'Dynamic
          :path $ :: 'Option 'Dynamic
          :data $ :: 'Option 'Dynamic
          :cursor $ :: 'Option 'Tag
          :enabled? $ :: 'Option 'Bool
          :fill-color $ :: 'Option (:: 'List 'Number)
          :line-color $ :: 'Option (:: 'List 'Number)
          :line-width $ :: 'Option 'Number
          :children $ :: 'Option (:: 'List 'Dynamic)
          :text-color $ :: 'Option (:: 'List 'Number)
          :text-size $ :: 'Option 'Number
    'calcit-paint.ui/touch-container $ {}
      :mode :ensure
      :kind :fn
      :doc "|Build a labelled touch-area container; children use scene coordinates and are drawn above the label. / 构建带标签的 touch-area 容器；children 使用场景坐标并绘制在标签之上。"
      :params $ [] 'options
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.ui/TouchContainerOptions
        :return $ :: 'Map 'Tag 'Dynamic
    'calcit-paint.ui/TextInputOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Options for an explicit :text-input focus area with an accessibility value. / 显式 :text-input focus area 的选项，带无障碍 value。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct TextInputOptions (:id 'String) (:label 'String) (:value 'String) (:x 'Number) (:y 'Number) (:dx 'Number) (:dy 'Number)
          :tab-index $ :: 'Option 'Number
          :action $ :: 'Option 'Dynamic
          :path $ :: 'Option 'Dynamic
          :data $ :: 'Option 'Dynamic
          :enabled? $ :: 'Option 'Bool
          :fill-color $ :: 'Option (:: 'List 'Number)
          :line-color $ :: 'Option (:: 'List 'Number)
          :line-width $ :: 'Option 'Number
          :text-color $ :: 'Option (:: 'List 'Number)
          :text-size $ :: 'Option 'Number
    'calcit-paint.ui/text-input-focus-area $ {}
      :mode :ensure
      :kind :fn
      :doc "|Build a focus-area text input with :text-input? true and a focusable :text-input accessibility node. / 构建 :text-input? true 且可聚焦 :text-input 无障碍节点的 focus-area 文本输入。"
      :params $ [] 'options
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.ui/TextInputOptions
        :return $ :: 'Map 'Tag 'Dynamic
    'calcit-paint.ui/IconLabelOptions $ {}
      :mode :ensure
      :kind :data
      :doc "|Options for a decorative icon + label group. / 装饰性 icon + label group 的选项。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct IconLabelOptions (:label 'String) (:x 'Number) (:y 'Number)
          :gap $ :: 'Option 'Number
          :icon-radius $ :: 'Option 'Number
          :icon-color $ :: 'Option (:: 'List 'Number)
          :text-color $ :: 'Option (:: 'List 'Number)
          :text-size $ :: 'Option 'Number
          :baseline $ :: 'Option 'Tag
    'calcit-paint.ui/icon-label $ {}
      :mode :ensure
      :kind :fn
      :doc "|Build a visual-only icon + label group; it adds no interaction or accessibility semantics. / 构建纯视觉 icon + label group；不添加交互或无障碍语义。"
      :params $ [] 'options
      :schema $ :: 'Fn $ {}
        :args $ [] 'calcit-paint.ui/IconLabelOptions
        :return $ :: 'Map 'Tag 'Dynamic
    'calcit-paint.ui/demo-scene $ {}
      :mode :ensure
      :kind :fn
      :doc "|Compose all scene builders into one validated scene that also works with render-to-png!. / 将所有 scene builder 组合为一个可校验、也可直接用于 render-to-png! 的场景。"
      :params $ []
      :schema $ :: 'Fn $ {}
        :args $ []
        :return $ :: 'Map 'Tag 'Dynamic
  :edges $ #{}
    :: :call 'calcit-paint.ui/demo-scene 'calcit-paint.ui/button
    :: :call 'calcit-paint.ui/demo-scene 'calcit-paint.ui/touch-container
    :: :call 'calcit-paint.ui/demo-scene 'calcit-paint.ui/icon-label
    :: :call 'calcit-paint.ui/demo-scene 'calcit-paint.ui/text-input-focus-area
    :: :type 'calcit-paint.ui/button 'calcit-paint.ui/ButtonOptions
    :: :type 'calcit-paint.ui/touch-container 'calcit-paint.ui/TouchContainerOptions
    :: :type 'calcit-paint.ui/text-input-focus-area 'calcit-paint.ui/TextInputOptions
    :: :type 'calcit-paint.ui/icon-label 'calcit-paint.ui/IconLabelOptions
