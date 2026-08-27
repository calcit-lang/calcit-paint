# C-safe blocking v1 migration / C-safe blocking v1 迁移

## 中文

- 将 `launch_canvas` 从 Rust trait-object ABI 迁移到 `launch_canvas_calcit_ffi_blocking_v1`，callback 通过 host table 同步调用，并由 host `free_buffer` 释放结果。
- 将 `push_drawing_data` 迁移到同步 buffer v1，crate 改为 `cdylib`，删除 legacy Rust ABI exports。
- 使用同一个可返回的 winit EventLoop 创建窗口和处理事件；Close/Escape 正常退出，不再调用 `process::exit`。
- 升级 Skia 0.70，使用当前 backend surface、font 与 flush API，并改善窗口、surface、锁和请求错误。
- 用 Calcit 0.13.56 将源码迁移到 canonical `calcit.cirru`，删除 `compact.cirru`、公开 Snapshot diff，并补齐函数/宏 schema。
- CI 固定 `setup-calcit@v1.3.0` 与 Calcit 0.13.56，覆盖 strict check、Rust tests/clippy、C symbol audit 和 Linux/Xvfb blocking GUI smoke。
- 保持事件模式兼容当前 stable clippy：已有 `..` 时不再保留冗余的字段通配绑定。
- macOS 依赖 Calcit host 在进程主线程执行 blocking FFI；Linux 明确使用 winit 的 `new_any_thread`，兼容 Calcit 的 worker-thread CLI 与 Xvfb。
- Skia GL interface 从 glutin 已激活的窗口 context 显式加载，避免 Linux EGL/GLX 环境下默认 native loader 与实际 context 不一致。

## English

- Migrated `launch_canvas` from the Rust trait-object ABI to `launch_canvas_calcit_ffi_blocking_v1`; callbacks invoke synchronously through the host table and release results through host `free_buffer`.
- Migrated `push_drawing_data` to synchronous buffer v1, changed the crate to `cdylib`, and removed legacy Rust-ABI exports.
- Use one returnable winit EventLoop for window creation and event handling; Close/Escape exits normally without `process::exit`.
- Upgraded to Skia 0.70 with current backend-surface, font, and flush APIs, while improving window, surface, lock, and request errors.
- Migrated source to canonical `calcit.cirru` with Calcit 0.13.56, retired `compact.cirru`, exposed Snapshot diffs, and added function/macro schemas.
- CI pins `setup-calcit@v1.3.0` and Calcit 0.13.56, covering strict checks, Rust tests/clippy, C symbol auditing, and a Linux/Xvfb blocking GUI smoke.
- Kept event patterns compatible with current stable clippy by removing redundant field wildcards when `..` already covers them.
- macOS relies on the Calcit host running blocking FFI on the process main thread; Linux explicitly uses winit's `new_any_thread` to support the worker-thread CLI and Xvfb.
- Load Skia's GL interface explicitly from the active glutin window context, avoiding native-loader/context mismatches across Linux EGL/GLX environments.
