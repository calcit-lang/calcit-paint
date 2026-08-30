# Native file drop events / 原生文件拖放事件

## 中文

- 在现有 winit 窗口链路上增加 `HoveredFile`、`DroppedFile` 与 `HoveredFileCancelled`，兼容 callback 分别收到 `:file-hover`、`:file-drop`、`:file-hover-cancel` map；事件携带最近的逻辑坐标与 modifier 状态。
- 扩展封闭的 `PaintEvent` / `PaintEventFfi` 协议到 30 个 variant。公开 hover/drop payload 使用 nominal `PaintFileEvent`，其中路径为 Calcit `FsPath`；取消事件使用不伪造路径的 `PaintFileHoverCancelEvent`。
- 原生边界严格要求 UTF-8 路径，无法无损表示的宿主路径会输出明确 stderr 诊断并跳过事件；Paint 不自动读取、复制或上传文件，文件系统副作用仍归应用所有。
- 默认 Calcit demo exhaustive `match` 已覆盖三类文件事件，并在窗口内实时显示拖入状态；新增 definition-attached 测试、可执行 example 与 Rust 正常/异常边界测试。
- 同步双语 README、架构 scaffold 与 roadmap；验证 Calcit 0.13.64、Rust tests/Clippy、release dylib、导出符号、离屏 PNG 和真实窗口 smoke。

## English

- Add `HoveredFile`, `DroppedFile`, and `HoveredFileCancelled` to the existing winit window path. Compatible callbacks receive `:file-hover`, `:file-drop`, and `:file-hover-cancel` maps with the latest logical coordinates and modifier state.
- Extend the closed `PaintEvent` / `PaintEventFfi` protocol to 30 variants. Public hover/drop payloads use nominal `PaintFileEvent` with a Calcit `FsPath`; cancellation uses `PaintFileHoverCancelEvent` without fabricating a path.
- Require lossless UTF-8 paths at the native boundary. Host paths that cannot be represented safely produce an explicit stderr diagnostic and no event. Paint does not read, copy, or upload files automatically; applications retain ownership of filesystem effects.
- Extend the default Calcit demo's exhaustive match with all three file events and show live ingress status in the window. Add a definition-attached test, an executable example, and Rust happy/error boundary tests.
- Synchronize the bilingual README, architecture scaffold, and roadmap; validate Calcit 0.13.64, Rust tests/Clippy, the release dylib, exported symbols, offscreen PNG, and a real-window smoke run.
