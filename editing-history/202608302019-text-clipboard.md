# Cross-platform text clipboard / 跨平台文本剪贴板

## 中文

- 新增强类型 Calcit API `read-clipboard-text! : () -> String` 与 `write-clipboard-text! : (String) -> Unit`，继续复用 C-safe buffer-v1 ABI，并对参数和平台错误给出明确诊断。
- 引入 `arboard 3.6.1`，关闭默认图片 feature，只保留 UTF-8 文本能力。使用进程内惰性 `Mutex<Option<Clipboard>>` 串行所有访问，保持 Linux X11/XWayland selection ownership，并在 winit event loop 退出时释放 handle。
- 本阶段不启用 compositor 支持不统一的纯 Wayland data-control，也不开放图片、HTML、文件列表、primary selection 或后台轮询。
- 默认可运行 Calcit demo 新增 `Shift+C` 写入与 `Shift+V` 读回，窗口内显示实时状态；CI 新增 Linux/Xvfb 真实剪贴板往返 smoke，并扩展 C export audit。
- 补充双语 README 与 architecture scaffold；验证 Calcit 0.13.64 canonical/check/quality/docs、Rust tests/strict Clippy、release dylib、公开 FFI 往返与 typed-window smoke。

## English

- Add typed Calcit APIs `read-clipboard-text! : () -> String` and `write-clipboard-text! : (String) -> Unit`, reusing the C-safe buffer-v1 ABI with strict arguments and explicit platform errors.
- Add `arboard 3.6.1` with its default image feature disabled, retaining only UTF-8 text behavior. A process-wide lazy `Mutex<Option<Clipboard>>` serializes access, preserves Linux X11/XWayland selection ownership, and releases the handle when the winit event loop exits.
- Do not enable compositor-dependent pure-Wayland data control in this phase, and do not expose images, HTML, file lists, primary selection, or background polling.
- Extend the runnable default Calcit demo with `Shift+C` write and `Shift+V` read-back shortcuts plus visible status. Add a real Linux/Xvfb clipboard round-trip smoke and extend the C export audit.
- Add bilingual README documentation and an architecture scaffold; validate Calcit 0.13.64 canonical/check/quality/docs, Rust tests/strict Clippy, the release dylib, public FFI round-trip, and typed-window smoke.
