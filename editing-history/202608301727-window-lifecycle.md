# Configurable window lifecycle / 可配置窗口生命周期

## 中文

- 保持 `launch-canvas!` 的标题、逻辑尺寸与 resizable 默认行为不变；新增 nominal `WindowOptions` 和 `launch-canvas-with-options!`，在 native 边界严格校验完整字段、有限正尺寸与最小尺寸约束。
- 新增 `set-window-title!`、`request-window-size!` 与 `close-window!`。请求先写入进程级 FIFO，在当前 callback 返回后的 event-loop 安全点串行应用，避免 callback 重入。
- 尺寸请求以 `:window-request` 事件区分 `:confirmed` 和 `:pending`；同步确认提供实际逻辑尺寸及 `:matched?`，异步平台以后续 `:resize` 为权威结果。
- `:resize` 新增 `:scale-factor`，缩放变化新增独立事件；窗口退出前只发送一次带明确 reason 的 `:window-close`。
- 对 nominal 启动配置、非法尺寸、请求顺序、重复启动、关闭幂等、scale factor、尺寸限制/异步确认及全部关闭原因增加 Rust 测试。
- 默认 Calcit demo 使用可配置启动 API，并提供可直接操作的 `T` 改标题、`S` 改尺寸、`Q` 关闭窗口；同步双语 README、架构 scaffold 与 CI 导出符号审计。

## English

- Preserve the title, logical size, and resizability defaults of `launch-canvas!`; add nominal `WindowOptions` and `launch-canvas-with-options!` with strict native validation of complete fields, finite positive dimensions, and minimum-size bounds.
- Add `set-window-title!`, `request-window-size!`, and `close-window!`. Requests enter a process-wide FIFO and are serialized at an event-loop safe point after the current callback returns, preventing callback re-entry.
- Report size requests as `:window-request` events with `:confirmed` or `:pending` status. Synchronous confirmation includes the actual logical size and `:matched?`; asynchronous platforms use the later `:resize` event as authoritative.
- Add `:scale-factor` to resize events, emit a dedicated scale-change event, and deliver exactly one reasoned `:window-close` before exit.
- Cover nominal startup configuration, invalid sizes, request ordering, duplicate launch, idempotent closing, scale factor, clamped/asynchronous size acknowledgement, and every close reason with Rust tests.
- Run the default Calcit demo through the configured launch API and expose interactive `T` title, `S` size, and `Q` close actions; update the bilingual README, architecture scaffold, and CI export audit.

## Verification / 验证

- Calcit 0.13.64 canonical Snapshot formatting, check-only, type/weak/deprecated/quality/dynamic-method analysis, Caps validation, and README format/check
- `cargo fmt --all -- --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- Release build, exported-symbol audit, and real configured-window smoke
