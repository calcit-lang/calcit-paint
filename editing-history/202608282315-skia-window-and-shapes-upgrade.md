# Skia、窗口层与图形能力升级

- 将 `skia-safe` 升级到 0.97.2，并迁移到 `winit` 0.30、`glutin` 0.32 和
  `glutin-winit` 0.5 的应用生命周期与 OpenGL surface API。
- 增加窗口缩放时的 GL/Skia surface 重建、逻辑像素坐标换算、滚轮事件、
  vsync，以及 OpenGL 到 GLES 的 context fallback。
- 扩展圆角矩形、椭圆、圆弧、矩形裁剪、组合透明度和显式 path close；
  对新增尺寸、半径与透明度参数增加边界校验。
- 修复文本对齐实际未参与绘制的问题；图片缓存现在根据文件大小和修改时间
  自动失效，避免每帧重复解码并支持资源热更新。
- 示例与 README 同步展示新增 shape，并将 Calcit/CI 基线更新到 0.13.58。
- 在 `.gitignore` 中忽略 macOS `.DS_Store`，避免本地桌面元数据进入提交。

验证包含 Rust 格式与静态检查、Calcit Snapshot 检查、README 代码块检查和
caps 契约检查。原生 Skia 链接测试依赖对应平台的预编译二进制；若镜像缺失，
需要取消 `SKIA_BINARIES_URL` 后从 rust-skia GitHub Release 获取。
