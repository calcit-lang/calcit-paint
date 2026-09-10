{}
  :schema-version 1
  :feature 'structured-scene-diagnostics
  :doc "|Expose stable machine-readable scene diagnostics while preserving the compatible string API. / 在保留兼容字符串 API 的同时公开稳定、机器可读的场景诊断。"
  :roots $ #{} 'calcit-paint.core/validate-scene-structured 'calcit-paint.creative-art/main!
  :definitions $ {}
    'calcit-paint.core/PaintSceneDiagnostic $ {}
      :mode :ensure
      :kind :data
      :doc "|Stable scene validation record; path and code are automation contracts while message may improve. / 稳定场景校验记录；path 与 code 是自动化契约，message 可持续改进。"
      :schema $ :: 'StructDef
      :code $ quote
        defstruct PaintSceneDiagnostic (:path 'String) (:code 'Tag)
          :field $ :: 'Option 'String
          :expected 'String
          :actual 'String
          :message 'String
    'calcit-paint.core/validate-scene-structured $ {}
      :mode :ensure
      :kind :fn
      :doc "|Return typed machine-readable diagnostics for a scene while leaving it unchanged. / 返回场景的强类型机器可读诊断，不修改输入场景。"
      :params $ [] 'scene
      :schema $ :: 'Fn $ {}
        :generics $ [] 'T
        :args $ [] 'T
        :return $ :: 'List 'calcit-paint.core/PaintSceneDiagnostic
    'calcit-paint.creative-art/validate-art-scene! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Validate the generated preview through the structured public boundary before launch. / 启动前通过公开结构化边界校验生成的预览。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
    'calcit-paint.creative-art/main! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Launch the runnable Creative Art Preview after structured scene validation. / 结构化场景校验后启动可运行的 Creative Art Preview。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
  :edges $ #{}
    :: :call 'calcit-paint.creative-art/main! 'calcit-paint.creative-art/validate-art-scene!
    :: :call 'calcit-paint.creative-art/validate-art-scene! 'calcit-paint.core/validate-scene-structured
    :: :type 'calcit-paint.core/validate-scene-structured 'calcit-paint.core/PaintSceneDiagnostic
