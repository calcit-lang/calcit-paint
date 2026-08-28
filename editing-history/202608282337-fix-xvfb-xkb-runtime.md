# 修复 Xvfb smoke 的 XKB 运行时依赖

- GitHub Actions 的构建、测试、Clippy 和动态库符号审计均通过，但升级到
  `winit 0.30` 后，Xvfb smoke 在启动 X11 键盘后端时无法动态加载
  `libxkbcommon-x11.so`。
- 在 CI 原生测试依赖中加入 Ubuntu 运行时包 `libxkbcommon-x11-0`，确保
  `xkbcommon-dl` 能在 Xvfb 环境加载 X11 集成库。
