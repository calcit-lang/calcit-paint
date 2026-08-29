# On-demand frame scheduling and animation timing / 按需帧调度与动画时钟

## 中文

- 新增 one-shot `request-frame!` Calcit API 与 `request_frame_calcit_ffi_v1` C-safe buffer 导出；无活跃 Paint 窗口时明确报错，重复请求会合并。
- 新增进程级帧调度状态与单调 `FrameClock`；`:frame` 事件提供帧序号、`timestamp-ms`、`delta-ms`、逻辑宽高及 scale factor。
- 窗口遮挡、最小化或 suspended 时不派发帧事件但保留至多一个请求；恢复后的首帧 delta 重置为零，event loop 结束时取消请求。
- 保持 winit 的按需 redraw 模型，没有启用常驻 `ControlFlow::Poll`；连续动画由 callback 显式请求下一帧。
- 默认 `calcit.cirru` 增加可运行圆形动画 demo：启动只请求一帧后空闲，按 `A` 开始或暂停逐帧动画。
- 增加调度生命周期、请求合并、无窗口调用、时钟恢复及 frame payload 测试，并同步双语 README 与 GitHub Actions 导出审计。
- 真实窗口 smoke 曾发现并修正 callback 重入 `launch-canvas!` 与 Calcit 调用多一层列表的问题；最终 smoke 确认帧 callback 正常交付并无泄漏退出。

## English

- Added the one-shot Calcit `request-frame!` API and C-safe `request_frame_calcit_ffi_v1` buffer export. Calls fail clearly without an active Paint window, and duplicate requests coalesce.
- Added process-wide scheduling state and a monotonic `FrameClock`; `:frame` events expose the frame number, `timestamp-ms`, `delta-ms`, logical dimensions, and scale factor.
- Occluded, minimized, and suspended windows stop dispatching frame events while retaining at most one request. The first restored frame resets delta to zero, and event-loop shutdown cancels pending work.
- Preserved winit's on-demand redraw model without permanent `ControlFlow::Poll`; continuous animations explicitly request the next frame from their callback.
- Extended the default `calcit.cirru` with a runnable circle animation: startup requests one frame and then idles, while `A` starts or pauses explicit frame chaining.
- Added coverage for scheduler lifecycle, coalescing, no-window requests, restored clock timing, and frame payloads, plus bilingual README documentation and GitHub Actions export auditing.
- Real-window smoke testing found and fixed both callback re-entry into `launch-canvas!` and an extra Calcit call-list layer; the final smoke confirms frame delivery and a leak-free exit.

## Verification / 验证

- `cargo fmt --all -- --check`
- `cargo test` (46 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README check-md (17 blocks), quality baseline, and dynamic-method gate
- Release `./build.sh`, macOS exported-symbol audit, and a real auto-exit Calcit window smoke with `request_frame_calcit_ffi_v1`
