# 升级共享 native FFI 0.1.2 / Upgrade shared native FFI 0.1.2

## 中文

- 升级至 `calcit_native_ffi 0.1.2`，同步共享 raw ABI contract。
- buffer 与 blocking-callback protocol 仍保持 v1；Skia/winit 事件循环、绘制状态与 callback 生命周期不变。
- 使用 Calcit 0.13.58 重新验证 typed wrapper、动态库 symbol 与 blocking canvas smoke。
- 将 winit 的正常 `LoopDestroyed` 生命周期事件显式视为无需 callback，避免退出时误报 unknown event。

## English

- Upgrade to `calcit_native_ffi 0.1.2` and synchronize the shared raw ABI contracts.
- Keep buffer and blocking-callback protocols at v1; the Skia/winit event loop, rendering state, and callback lifecycle are unchanged.
- Revalidate the typed wrapper, dylib symbols, and blocking canvas smoke with Calcit 0.13.58.
- Treat winit's normal `LoopDestroyed` lifecycle event as callback-free to avoid a misleading unknown-event message during shutdown.
