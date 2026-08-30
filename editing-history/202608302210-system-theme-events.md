# Typed system theme events / 强类型系统主题事件

## 中文

- 新增 `:window-theme` 兼容 map 事件与 nominal `PaintWindowThemeEvent`，其字段为严格校验的 `:theme`（`:light`、`:dark`、`:unknown`）和 `:initial?` Bool。
- 保持 `:ready` 先行；首次系统主题观测延后到首帧绘制完成后再发送，避免在同一绘制批次重复入队 scene，导致 focus area 重复注册。
- 运行期 winit `ThemeChanged` 会发送同一事件并请求 redraw；Calcit typed decoder 对未知 theme tag 直接失败，保留旧 callback 的 map 兼容性。
- 默认可运行 demo 根据主题更新背景、标题强调色与状态文字；补充 Rust 映射/transport 测试、Calcit decoder 测试、双语 README 与 architecture scaffold。

## English

- Add the compatible `:window-theme` map event and nominal `PaintWindowThemeEvent`, with a strictly validated `:theme` (`:light`, `:dark`, or `:unknown`) and `:initial?` Bool.
- Preserve `:ready` first; emit the initial system-theme observation only after the first scene draw, avoiding two scene batches in one paint pass and duplicate focus-area registration.
- Runtime winit `ThemeChanged` emits the same event and requests redraw; the typed Calcit decoder fails on unknown theme tags while legacy callbacks keep their map compatibility.
- The runnable default demo updates its background, title accent, and status text from the theme; add Rust mapping/transport tests, a Calcit decoder test, bilingual README documentation, and an architecture scaffold.
