{}
  :schema-version 1
  :feature 'creative-art-preview
  :doc "|Provide a complete fixed-seed Creative Art scene, on-demand animation controls, and deterministic PNG export using only public Calcit Paint APIs. / 仅使用公开 Calcit Paint API 提供完整的固定 seed Creative Art 场景、按需动画控制与确定性 PNG 导出。"
  :roots $ #{} 'calcit-paint.creative-art/main! 'calcit-paint.creative-art/export-art-frame!
  :definitions $ {}
    'calcit-paint.creative-art/*seed $ {}
      :mode :ensure
      :kind :data
      :doc "|Current deterministic artwork seed. / 当前确定性作品 seed。"
      :schema $ :: 'Ref 'Number
      :code $ quote
        defatom *seed 17
    'calcit-paint.creative-art/*time-ms $ {}
      :mode :ensure
      :kind :data
      :doc "|Current animation time from the Paint frame clock. / 来自 Paint frame clock 的当前动画时间。"
      :schema $ :: 'Ref 'Number
      :code $ quote
        defatom *time-ms 0
    'calcit-paint.creative-art/*playing? $ {}
      :mode :ensure
      :kind :data
      :doc "|Whether the one-shot frame chain is active. / one-shot frame 链是否活跃。"
      :schema $ :: 'Ref 'Bool
      :code $ quote
        defatom *playing? false
    'calcit-paint.creative-art/*export-status $ {}
      :mode :ensure
      :kind :data
      :doc "|Visible export status for the runnable studio. / 可运行 studio 中显示的导出状态。"
      :schema $ :: 'Ref 'String
      :code $ quote
        defatom *export-status "|Press P to export / 按 P 导出"
    'calcit-paint.creative-art/seeded-unit $ {}
      :mode :ensure
      :kind :fn
      :doc "|Return a deterministic number in [0, 1] for one seed/index pair. / 为一组 seed/index 返回 [0, 1] 内的确定性数字。"
      :params $ [] 'seed 'index
      :schema $ :: 'Fn $ {}
        :args $ [] 'Number 'Number
        :return 'Number
    'calcit-paint.creative-art/build-art-scene $ {}
      :mode :ensure
      :kind :fn
      :doc "|Build the open scene-map boundary shared by the window and offscreen export. / 构建窗口与离屏导出共享的开放 scene-map 边界。"
      :params $ [] 'seed 'time-ms 'export-status 'controls?
      :schema $ :: 'Fn $ {}
        :args $ [] 'Number 'Number 'String 'Bool
        :return $ :: 'Map 'Tag 'Dynamic
    'calcit-paint.creative-art/render-art! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Render the current creative state into the active Paint window. / 将当前创作状态绘制到活跃 Paint 窗口。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
    'calcit-paint.creative-art/export-art-frame! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Export one explicit seed/time frame through the public offscreen API. / 通过公开离屏 API 导出显式 seed/time frame。"
      :params $ [] 'path 'seed 'time-ms
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ [] 'String 'Number 'Number
    'calcit-paint.creative-art/export-current! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Export the current artwork to creative-art.png. / 将当前作品导出到 creative-art.png。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
    'calcit-paint.creative-art/toggle-animation! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Pause or resume explicit one-shot frame chaining. / 暂停或继续显式 one-shot frame 链。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
    'calcit-paint.creative-art/regenerate! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Advance the deterministic seed and redraw. / 递增确定性 seed 并重绘。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
    'calcit-paint.creative-art/handle-event! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Drive creative controls from the closed nominal PaintEvent protocol. / 通过封闭 nominal PaintEvent 协议驱动创作控制。"
      :params $ [] 'event
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ [] 'calcit-paint.core/PaintEvent
    'calcit-paint.creative-art/main! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Launch the runnable Creative Art Preview. / 启动可运行的 Creative Art Preview。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
    'calcit-paint.creative-art/reload! $ {}
      :mode :ensure
      :kind :fn
      :doc "|Redraw the artwork after a Calcit reload. / Calcit reload 后重绘作品。"
      :params $ []
      :schema $ :: 'Fn $ {} (:return 'Unit)
        :args $ []
  :edges $ #{}
    :: :call 'calcit-paint.creative-art/main! 'calcit-paint.creative-art/handle-event!
    :: :call 'calcit-paint.creative-art/handle-event! 'calcit-paint.creative-art/render-art!
    :: :call 'calcit-paint.creative-art/handle-event! 'calcit-paint.creative-art/toggle-animation!
    :: :call 'calcit-paint.creative-art/handle-event! 'calcit-paint.creative-art/regenerate!
    :: :call 'calcit-paint.creative-art/handle-event! 'calcit-paint.creative-art/export-current!
    :: :call 'calcit-paint.creative-art/render-art! 'calcit-paint.creative-art/build-art-scene
    :: :call 'calcit-paint.creative-art/export-art-frame! 'calcit-paint.creative-art/build-art-scene
    :: :call 'calcit-paint.creative-art/export-current! 'calcit-paint.creative-art/export-art-frame!
