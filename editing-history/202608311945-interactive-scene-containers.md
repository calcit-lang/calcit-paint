# Interactive scene containers / 交互场景容器

## Summary / 概要

- `touch-area` and `focus-area` now accept `:children`, so an interactive scene
  node can paint text, icons, and nested controls without a separate sibling
  group.
- `touch-area` 与 `focus-area` 现在支持 `:children`，交互场景节点可直接承载
  文本、图标和嵌套控件，无需再使用独立的 sibling group。

## Implementation notes / 实现要点

- Paint and register the outer interactive node before recursively painting its
  children. Existing reverse-order hit testing then makes a later-painted,
  overlapping nested node win naturally for both pointer and focus selection.
- 先绘制并注册外层交互节点，再递归绘制 children。既有的逆序 hit-test 因而会自然地
  让视觉上更靠前的重叠嵌套节点在 pointer 与 focus 选择中胜出。
- Pass the active transform, clip stack, render mode, and compositing context
  unchanged into children. Container `:position` remains its own hit geometry,
  not a local coordinate transform for children.
- 向 children 原样传递当前 transform、clip stack、render mode 与合成上下文。
  容器的 `:position` 仍只描述自身 hit geometry，不是 children 的局部坐标变换。
- Keep `cached-group`'s recursive interactive-descendant rejection. It already
  walks groups and recognises touch/focus nodes, so cached subtrees cannot hide
  interaction introduced through `:children`.
- 保持 `cached-group` 对交互后代的递归拒绝。它已遍历 group 并识别 touch/focus
  节点，因此不能通过 `:children` 将交互行为隐藏进缓存子树。

## Verification / 验证

- Rust tests cover child-over-container pixels, nested touch and focus hit
  precedence, parsing, and cached-subtree rejection.
- Rust 测试覆盖 child 覆盖容器像素、嵌套 touch/focus 命中优先级、解析与缓存子树拒绝。
- The default `calcit.cirru` demo includes a nested touch action and a focusable
  container with a circle and label; the release native smoke test rendered it.
- 默认 `calcit.cirru` demo 包含嵌套 touch action 与带圆形图标和标签的可聚焦容器；
  release native smoke 已完成渲染验证。
