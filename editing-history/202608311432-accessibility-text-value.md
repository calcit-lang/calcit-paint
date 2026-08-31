# Accessible text value actions / 无障碍文本值动作

- 为启用的、显式标注 `:role :text-input` 且位于 `focus-area` 的语义节点注册 AccessKit `SetValue`。其他 role、非 focus-area 与 disabled 节点不会声明该动作。
- 接收 `ActionData::Value` 后，native 层只通过既有 `accessibility-action` event 传递 `:operation :set-value` 与字符串 `:value`；不会直接写入 Calcit 应用状态。缺少或错误 action data 会安全忽略。
- `PaintAccessibilityActionEvent` 新增 `Option<String>` `:value`；Calcit decoder 仅允许 `:set-value` 带值，并保留 `:focus` / `:activate` 兼容。可运行 demo 用 typed `Ref<String>` atom 接收新值并下一帧重新绘制语义值。
- 增加 AccessKit node、native event payload 与 Calcit decoder 测试；同步中英 README 与架构计划。验证 Rust tests (93)、strict Clippy、Calcit canonical/check/quality/docs、release dylib 与 native smoke。

## Knowledge / 知识点

AccessKit 的 `SetValue` 请求必须同时检查 `Action::SetValue`、目标 semantic role 与 `ActionData::Value`。仅添加 action 不代表应用值会自动变化：系统 adapter 只能投递请求，Calcit 应用必须消费 typed event、更新自身 state，并在随后 redraw 重新发布 `Node::set_value`。这保持了 UI state、渲染和可访问性语义的单一所有权。

## English summary

- Register AccessKit `SetValue` only for enabled semantic nodes explicitly marked `:role :text-input` and attached to a `focus-area`. Other roles, non-focus areas, and disabled nodes never publish it.
- On `ActionData::Value`, native code emits the existing `accessibility-action` event with `:operation :set-value` and a string `:value`; it never mutates Calcit application state. Missing or invalid action data is ignored safely.
- `PaintAccessibilityActionEvent` gains `Option<String>` `:value`; the Calcit decoder accepts `:set-value` only when a value exists and preserves `:focus` / `:activate`. The runnable demo stores the new value in a typed `Ref<String>` atom and redraws the semantic value next frame.
- Add AccessKit-node, native-event-payload, and Calcit-decoder tests; synchronize bilingual README and architecture plan. Verified Rust tests (93), strict Clippy, Calcit canonical/check/quality/docs, release dylib, and native smoke.
