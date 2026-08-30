# Nominal typed Paint events / Nominal 强类型 Paint 事件

## 中文

- 新增公开 `PaintEvent` enum 与按领域拆分的 nominal payload struct；可选协议字段统一使用 `Option<T>`，应用自定义的 `:action`、`:path`、`:data` 仅在 `PaintTarget` 中保留为 `Option<Dynamic>`。
- 新增 `launch-canvas-typed!` 和 blocking-v1 原生导出。Rust 侧把兼容事件规范化为私有 `PaintEventFfi<Map<Tag, Dynamic>>` envelope，移除 `nil` 可选字段、嵌套 target，并把 window acknowledgement 拆成独立 variant；Calcit 侧立即用 `decode-map-as` 严格恢复公开 nominal 类型。
- 保持 `launch-canvas!` 与 `launch-canvas-with-options!` 的旧版 `nil` / map callback 完全兼容；未知事件、未知 window operation、错误 tag 或 payload 形状会明确失败。
- 默认可运行 Calcit demo 已迁到 `launch-canvas-typed!`，通过 exhaustive `match` 覆盖 27 个 variant；definition-attached 测试覆盖 frame 与 pointer target 解码，Rust 测试覆盖启动、Option 规范化、window 拆分与异常分支。
- 同步双语 README、架构 scaffold、质量基线及 CI 导出符号审计。质量基线只新增 `paint-event-from-ffi` 的一个原始 `Map<Tag, Dynamic>` FFI 解码边界。

## English

- Add the public `PaintEvent` enum and domain-specific nominal payload structs. Optional protocol fields use `Option<T>`; application-defined `:action`, `:path`, and `:data` remain open only as `Option<Dynamic>` fields on `PaintTarget`.
- Add `launch-canvas-typed!` and its blocking-v1 native export. Rust normalizes compatible events into a private `PaintEventFfi<Map<Tag, Dynamic>>` envelope, removes nil optionals, nests the target, and splits window acknowledgements into distinct variants; Calcit immediately restores strict public nominal values with `decode-map-as`.
- Preserve the legacy `nil` / map callback contracts of `launch-canvas!` and `launch-canvas-with-options!`. Unknown events, unsupported window operations, invalid tags, and malformed payloads fail explicitly.
- Migrate the default runnable Calcit demo to `launch-canvas-typed!` with an exhaustive match over all 27 variants. Definition-attached tests cover frame and pointer-target decoding; Rust tests cover startup, Option normalization, window splitting, and error paths.
- Update the bilingual README, architecture scaffold, quality baseline, and CI export audit. The baseline adds only the raw `Map<Tag, Dynamic>` FFI decode boundary on `paint-event-from-ffi`.

## Verification / 验证

- Calcit 0.13.64 canonical Snapshot formatting, check-only, attached unit tests, example/type/weak/deprecated/quality/dynamic-method analysis, Caps validation, and README format/check
- `cargo fmt --all -- --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- Release build, exported-symbol audit, offscreen PNG smoke, and real typed-window smoke
