# Accessible text selection and replacement / 无障碍文本选择与替换

## English

- Add strict, Unicode-safe selection metadata for `:role :text-input` accessibility annotations: optional `:selection-start` and `:selection-end` are non-negative Unicode scalar indices into `:value` forming a half-open `[start, end)` range (`start == end` is a caret).
- Reject invalid metadata: the pair is required together, requires `:value`, only applies to text inputs, and fails when `start > end` or `end` exceeds the value's Unicode scalar length.
- Publish AccessKit `SetTextSelection` and `ReplaceSelectedText` alongside `SetValue` for enabled text-input focus areas. Each text input exposes a `TextRun` child carrying the value and per-scalar character lengths, and the node publishes the current `text_selection`.
- Translate platform requests to typed `(:accessibility-action payload)` events as `:set-text-selection` and `:replace-selected-text` with `:selection-start`/`:selection-end` (`Option<Number>`) and `:text` (`Option<String>`). Native code never changes application state.
- Safely reject invalid ranges, wrong action data, non-text-input or disabled nodes, and mismatched text-run identities before emitting any event.
- Extend `PaintAccessibilityActionEvent`, `paint-event-from-ffi`, and the runnable `calcit-paint.main` demo, which now keeps value plus selection state and renders the selected range for Focus A.
- Add Rust unit tests for metadata validation, semantic-tree text-run/selection construction, action event shaping, and text-selection validation, plus Calcit decode tests.

Validation:

- `calcit ./calcit.cirru edit format` canonical with zero unintended diff; `--check-only` passes
- `analyze quality --baseline config/calcit-quality.json` unchanged at 11 schema-dynamic positions; zero dynamic-method findings
- `calcit docs format-md`/`check-md` for README; `calcit test --require-match` (16 passed); all eight `check-examples` targets
- `./scripts/check-cookbook.sh` and `./scripts/check-creative-art.sh`
- `cargo fmt --check`, 101 Rust tests, and strict Clippy

## 中文

- 为 `:role :text-input` 无障碍标注新增严格、Unicode-safe 的 selection metadata：可选 `:selection-start` 与 `:selection-end` 是对 `:value` 的非负 Unicode scalar 索引，构成半开区间 `[start, end)`（`start == end` 表示 caret）。
- 拒绝非法 metadata：两字段必须同时出现、依赖 `:value`、仅适用于 text-input；当 `start > end` 或 `end` 超过 value 的 Unicode scalar 长度时报错。
- 对启用的 text-input focus area，在 `SetValue` 之外发布 AccessKit `SetTextSelection` 与 `ReplaceSelectedText`。每个 text input 暴露携带 value 与逐 scalar 字符长度的 `TextRun` 子节点，并发布当前 `text_selection`。
- 将平台请求转换为强类型 `(:accessibility-action payload)` 事件：`:set-text-selection` 与 `:replace-selected-text`，携带 `:selection-start`/`:selection-end`（`Option<Number>`）与 `:text`（`Option<String>`）。native 层绝不修改应用状态。
- 在发出任何事件前安全拒绝非法 range、错误 action data、非 text-input 或 disabled 节点，以及不匹配的 text-run 身份。
- 扩展 `PaintAccessibilityActionEvent`、`paint-event-from-ffi` 与可运行 `calcit-paint.main` demo；demo 现在维护 value 与 selection 状态，并渲染 Focus A 的选区。
- 增加 Rust 单元测试（metadata 校验、语义树 text-run/selection 构建、action 事件结构、text-selection 校验）与 Calcit 解码测试。

验证：

- `calcit ./calcit.cirru edit format` canonical 零意外 diff；`--check-only` 通过
- `analyze quality --baseline config/calcit-quality.json` 维持 11 个 schema-dynamic 位置；动态方法零命中
- README 的 `calcit docs format-md`/`check-md`；`calcit test --require-match`（16 passed）；全部 8 个 `check-examples` target
- `./scripts/check-cookbook.sh` 与 `./scripts/check-creative-art.sh`
- `cargo fmt --check`、101 个 Rust 测试与 strict Clippy

Related: #82, #80, #45.
