# Pointer and keyboard event semantics

## 中文

- 用 `InputState` 集中保存逻辑指针位置、当前 modifier、每个鼠标按键最近 click record 与最近 click count，替代仅保存坐标的 `RefCell`。
- mouse-down/up 新增可移植 button tag；多击按 500 ms 与四个逻辑像素阈值、按 button 独立计数；mouse-up 读取对应 button 的 count，避免被其他按键覆盖。
- 所有 pointer/keyboard 事件增加明确的 `:modifiers` map；键盘事件保留旧数字 key-code 并增加 `:physical-key`，用于区分布局相关 logical key 与物理键位。
- touch-area drag 记录启动 button；离开窗口发送带 `:cancelled? true` 的 mouse-leave 并清理追踪，防止窗口外松开后留下拖拽状态。
- 默认 Calcit 场景增加可点击、可拖拽且可按 `I` 的 input demo，回调会把实时 event map 输出到 terminal。README 提供中英双语字段说明和跨平台约束。

## English

- Centralized logical pointer position, modifier state, per-button click records, and the latest click count in `InputState`, replacing the coordinate-only `RefCell`.
- Added portable button tags to mouse down/up. Multi-clicks are tracked per button using a 500 ms/four-logical-pixel threshold; mouse-up retrieves its own button count.
- Added a precise `:modifiers` map to pointer and keyboard events. Keyboard events retain the legacy numeric key code and add `:physical-key` to distinguish logical and physical keys.
- Recorded the initiating button for touch-area drags. Leaving the window emits `:mouse-leave` with `:cancelled? true` and clears tracking so an outside release cannot leave a stuck drag.
- Added a clickable, draggable, and `I`-key-driven input demo to the default Calcit scene; its callback prints live event maps. The README now documents fields and cross-platform rules in Chinese and English.
