# Hover, pointer capture, and system cursor semantics / 悬停、指针捕获与系统光标语义

## 中文

- `touch-area` 新增严格 tag 类型的可选 `:cursor`，映射到 winit `CursorIcon`；省略/`nil` 使用默认光标，字符串和未知 tag 会被 scene validation 拒绝。
- hit testing 继续采用最后绘制节点优先；每个交互 shape 使用 scene 结构路径与 event target 作为稳定 identity，跨重绘保持 hover，同路径目标替换也会产生清晰转换。
- 新增 `:pointer-enter` / `:pointer-leave`，携带位置、修饰键、target、cursor 与 `:captured? false`；重叠目标、目标移除及窗口离开均有确定事件顺序。
- 现有 drag 状态升级为明确 pointer capture：按下后移动与对应释放继续路由到原目标，并在 `mouse-down` / `mouse-move` / `mouse-up` 上提供严格 Bool `:captured?`。
- 目标移除、窗口离开与窗口失焦分别发送 reason 为 `:target-removed`、`:window-leave`、`:window-blur` 的 `:pointer-cancel`；兼容的 `:mouse-leave` 行为保留。
- pointer/capture 状态改为 event-loop 线程本地数据，符合 winit 与 blocking callback 生命周期，并消除 Rust 并行测试之间的全局状态串扰。
- 默认 `calcit.cirru` 增加可运行的重叠 `:grab` / `:crosshair` demo；状态更新通过 one-shot `request-frame!` 合并，避免事件 burst 重复排队 scene。
- 双语 README 记录 cursor tag、hover 顺序、capture/release/cancel 以及非物理光标 confinement 的边界。

## English

- Added strict optional `:cursor` tags to `touch-area`, mapped to winit `CursorIcon`. Missing/`nil` uses the default cursor, while strings and unknown tags fail scene validation.
- Hit testing remains last-drawn-first. Each interactive shape uses its scene structural path plus event target as stable identity, preserving hover across redraws while making same-path target replacement explicit.
- Added `:pointer-enter` and `:pointer-leave` with position, modifiers, target, cursor, and `:captured? false`; overlaps, removal, and window exit now have deterministic ordering.
- Upgraded the existing drag state into explicit pointer capture. Movement and the matching release stay routed to the pressed target, with a strict Boolean `:captured?` on mouse down/move/up events.
- Target removal, window exit, and focus loss emit `:pointer-cancel` with reasons `:target-removed`, `:window-leave`, and `:window-blur`; the compatible `:mouse-leave` contract remains.
- Moved pointer/capture state to event-loop thread-local storage, matching the winit/blocking-callback lifecycle and eliminating global-state interference between parallel Rust tests.
- Extended the default `calcit.cirru` with a runnable overlapping `:grab` / `:crosshair` demo. Status updates coalesce through one-shot `request-frame!` calls instead of queueing duplicate scenes during event bursts.
- Documented cursor tags, hover ordering, capture/release/cancel behavior, and the non-confining capture boundary in the bilingual README.

## Verification / 验证

- `cargo fmt --all -- --check`
- `cargo test` (50 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README check-md (18 blocks), quality baseline, and dynamic-method gate
- Release `./build.sh` and a real auto-exit macOS Calcit window smoke; the same runnable scene remains covered by the existing Linux/Xvfb Actions smoke
