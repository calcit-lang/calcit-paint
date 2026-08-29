# Focus, shortcut chords, and IME / 焦点、快捷键与 IME

## 中文

- 新增稳定 `:focus-id` 驱动的 `:focus-area`（兼容 `:focusable` 别名），支持点击聚焦、
  `:tab-index` 顺序与循环 Tab/Shift+Tab 导航、负 index 排除、节点卸载清理和重复 ID 报错。
- 新增 `focus!`、`blur!`、`focused?` 三个 Calcit wrapper 与 C-safe buffer ABI；程序化焦点事件
  会排队到当前 callback 返回后再投递，避免重入。
- `:focus-in` / `:focus-out` 事件携带 owner、related owner、原因和原有 target 数据；Escape、
  窗口失焦和节点卸载都有明确的清理语义。
- `:key-listener` 新增可选精确 modifier chord 与 focus scope。省略 modifier 继续保持旧版 wildcard
  行为，显式 map 则精确匹配四个 modifier flag。
- 将 winit IME 映射为 enabled、composition start/update/end、committed text-input 与 disabled
  生命周期；preedit cursor 保留 UTF-8 byte index，所有失焦路径会取消活动 composition。
- 默认 Calcit scene 新增两个可交互输入区域、Tab/Shift+Tab、焦点限定 Enter 与调用 `focus!` 的
  Shift+K 快捷键，能够直接运行观察事件 map。

## English

- Added stable `:focus-id`-based `:focus-area` nodes (with the `:focusable` alias), pointer focus,
  ordered and wrapping Tab/Shift+Tab traversal, negative-index exclusion, removal cleanup, and
  duplicate-ID validation.
- Added the `focus!`, `blur!`, and `focused?` Calcit wrappers with C-safe buffer ABI exports.
  Programmatic transitions are queued until the current callback returns to avoid reentrancy.
- Added `:focus-in` / `:focus-out` events with owner, related owner, reason, and existing target data,
  plus explicit cleanup for Escape, window blur, and removed nodes.
- Extended `:key-listener` with optional exact modifier chords and focus scope. Omitted modifiers retain
  the legacy wildcard behavior; an explicit map matches all four modifier flags exactly.
- Mapped winit IME input into enabled, composition start/update/end, committed text-input, and disabled
  lifecycle events. Preedit cursors preserve UTF-8 byte indices and every focus-loss path cancels a
  live composition.
- Extended the default runnable Calcit scene with two interactive input areas, Tab/Shift+Tab,
  focus-scoped Enter, and a Shift+K shortcut that calls `focus!`, exposing live event maps directly.

## Verification / 验证

- `cargo fmt --all -- --check`
- `cargo test` (39 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README example check, quality baseline, and dynamic-method gate
- `./build.sh`, C-safe export audit, and real macOS `CALCIT_PAINT_SMOKE_ONCE=1` default-demo smoke
