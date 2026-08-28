# 采用共享 native FFI crate / Adopt the shared native FFI crate

## 中文

- 使用 `calcit_native_ffi 0.1.0` 替换本地 buffer/blocking-v1 协议、EDN transport 和 adapter 模板。
- 升级到 `cirru_edn 0.8`，适配 owned list/map view 和 Arc-backed string/tag 类型。
- 模块仍负责 Skia/winit 桌面事件循环、绘制队列、shape 解析和 callback 调度。
- 将 Calcit/CI 基线升级到 0.13.57，并保留 C-safe symbol audit 与 Xvfb blocking smoke。
- 补充中英双语的责任边界与维护文档。

## English

- Replace the local buffer/blocking-v1 protocol, EDN transport, and adapter template with `calcit_native_ffi 0.1.0`.
- Upgrade to `cirru_edn 0.8` and adapt owned list/map views plus Arc-backed string/tag types.
- Keep the Skia/winit desktop event loop, drawing queue, shape decoding, and callback scheduling module-owned.
- Raise the Calcit/CI baseline to 0.13.57 while retaining the C-safe symbol audit and Xvfb blocking smoke.
- Add bilingual responsibility-boundary and maintenance documentation.
