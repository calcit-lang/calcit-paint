# 维护指南 / Maintainer guide

## 中文

- 修改 `calcit.cirru` 前先运行 `calcit docs agents --full`，并使用 `calcit edit` / `calcit tree`。
- C-safe buffer/blocking ABI 与 Cirru EDN transport 来自 `calcit_native_ffi`；本仓库只维护 Skia/winit 绘制、事件循环和 shape 解析。
- 提交前运行 Rust fmt/test/strict clippy、`caps --strict --ci`、Calcit check，并在 Linux/Xvfb 或真实桌面环境执行 blocking canvas smoke。
- GitHub Actions 的第三方 `uses:` 使用精确正式 release tag；维护者已接受 tag 可移动的风险。保留最小权限及原功能、类型和渲染门禁，不另加 pin 校验脚本。
- 不要把窗口事件循环、绘制状态或回调生命周期下沉到共享 FFI crate。

## English

- Run `calcit docs agents --full` before editing `calcit.cirru`, then use `calcit edit` / `calcit tree`.
- `calcit_native_ffi` owns the C-safe buffer/blocking ABI and Cirru EDN transport; this repository owns Skia/winit rendering, the event loop, and shape decoding.
- Before committing, run Rust fmt/tests/strict clippy, `caps --strict --ci`, the Calcit check, and a blocking canvas smoke under Linux/Xvfb or a real desktop session.
- Third-party GitHub Actions `uses:` entries use exact formal release tags; the maintainer accepts mutable-tag risk. Preserve minimum permissions and existing functional, type and rendering gates without an extra pin-check script.
- Do not move the window event loop, rendering state, or callback lifecycle into the shared FFI crate.
