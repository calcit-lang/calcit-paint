# Calcit Paint cookbook / Calcit Paint 场景手册

This is the smallest executable starting point for each public scene capability.
The default `calcit-paint.main/render!` remains the integration demo; use this
cookbook when copying one focused pattern into an application or an Agent task.

这是每项公开场景能力的最小可执行起点。默认的
`calcit-paint.main/render!` 仍是集成 demo；在应用或 Agent 任务中复制一个聚焦模式时，
请使用本手册。

## First run / 首次运行

```bash
./build.sh
calcit ./calcit.cirru
```

For a fast non-interactive validation loop, run the checked cookbook smoke:

快速进行非交互验证时，运行受 CI 检查的 cookbook smoke：

```bash
./scripts/check-cookbook.sh
```

## Stable index / 稳定索引

| Capability / 能力 | Minimal entry / 最小入口 | Verify / 验证 |
| --- | --- | --- |
| Basic shapes / 基础图元 | `validate-scene` + `:rectangle` | `validate-scene` returns `[]` / 返回 `[]` |
| Group, transform, clip / group、变换、裁剪 | `:group`, `:translate`, `:clip-rect` | `validate-scene` + offscreen PNG |
| Touch and focus / touch 与 focus | `:touch-area`, `:focus-area` | native demo or Xvfb smoke / 原生或 Xvfb smoke |
| Typed events / 强类型事件 | `launch-canvas-typed!`, `PaintEvent` | exhaustive `match` compiles / 穷尽 `match` 可编译 |
| Accessibility / 无障碍 | `:accessibility` on a touch/focus area | native semantic-tree smoke / 原生语义树 smoke |
| Offscreen snapshots / 离屏快照 | `render-to-png!` | PNG signature and pixel tests / PNG 签名与像素测试 |
| Local assets / 本地资源 | `:image` | `validate-scene` then native/offscreen render / 先校验再原生/离屏绘制 |

Use `calcit ./calcit.cirru query examples calcit-paint.core/validate-scene`
for an API-attached validation recipe. This document is the stable index for
the remaining cookbook recipes, including explicit-write offscreen export.

使用 `calcit ./calcit.cirru query examples calcit-paint.core/validate-scene`
获取附着在 API 上的校验示例。本文档是其余 cookbook recipe（包括显式写入的离屏导出）
的稳定索引。

## 1. Basic shape / 基础图元

Expected result: an empty diagnostic list. / 预期结果：空诊断列表。

```cirru.no-check
ns cookbook.basic $ :require
  calcit-paint.core :refer $ validate-scene

validate-scene $ {} (:type :rectangle)
  :position $ [] 20 20
  :width 120
  :height 48
  :fill-color $ [] 210 72 48
```

## 2. Group, transform, and clip / group、变换与裁剪

Expected result: the circle is visible only inside the rounded card. /
预期结果：圆形只在圆角卡片内部可见。

```cirru.no-check
ns cookbook.clip $ :require
  calcit-paint.core :refer $ validate-scene

validate-scene $ {} (:type :clip-rounded-rect)
  :position $ [] 20 20
  :width 180
  :height 96
  :radius 16
  :children $ []
    {} (:type :translate) (:x 40) (:y 0)
      :children $ []
        {} (:type :circle)
          :position $ [] 20 68
          :radius 48
          :fill-color $ [] 280 74 54
```

## 3. Touch and focus containers / touch 与 focus 容器

Expected result: nested interactive children paint later and win overlapping
hits. The container position is hit geometry, not a local origin for children.

预期结果：嵌套交互 child 后绘制，并在重叠命中中胜出。容器 position 是 hit geometry，
不是 children 的局部原点。

```cirru.no-check
ns cookbook.interaction $ :require
  calcit-paint.core :refer $ validate-scene

validate-scene $ {} (:type :touch-area) (:dx 120) (:dy 28)
  :position $ [] 180 80
  :action :outer
  :children $ []
    {} (:type :text) (:text "|Outer container")
      :position $ [] 120 80
      :color $ [] 0 0 96
      :size 14
      :align :center
    {} (:type :touch-area) (:dx 32) (:dy 18)
      :position $ [] 260 80
      :action :inner
      :children $ []
        {} (:type :text) (:text |Inner)
          :position $ [] 260 80
          :color $ [] 0 0 96
          :size 12
          :align :center
```

Run `calcit ./calcit.cirru` and use the default scene's nested touch and focus
containers for a real event-loop check. In CI, `CALCIT_PAINT_SMOKE_ONCE=1`
covers the same native entrypoint.

运行 `calcit ./calcit.cirru`，使用默认场景中的嵌套 touch/focus 容器进行真实
event-loop 检查。CI 使用 `CALCIT_PAINT_SMOKE_ONCE=1` 覆盖同一原生入口。

## 4. Typed event callback / 强类型事件回调

Use the typed entrypoint for new applications and keep the `match` exhaustive.
Use the default demo as the runnable full protocol reference.

新应用使用 typed 入口，并保持 `match` 穷尽。默认 demo 是可运行的完整协议参考。

```cirru.no-check
launch-canvas-typed! (WindowOptions :title |Cookbook :width 640 :height 420)
  fn (event)
    match event
      (:pointer payload)
        println $ :kind payload
      (:focus payload)
        println $ :kind payload
      (:accessibility-action payload)
        println $ :operation payload
      _ $ println event
```

## 5. Explicit accessibility / 显式无障碍语义

Add semantics only to an interactive area. A `:focusable? true` annotation must
be attached to `:focus-area`; a touch-only button must omit it.

只为交互 area 添加语义。`:focusable? true` 标注必须挂在 `:focus-area`；纯 touch
按钮必须省略它。

```cirru.no-check
ns cookbook.accessibility $ :require
  calcit-paint.core :refer $ validate-scene

validate-scene $ {} (:type :focus-area) (:focus-id |editor) (:text-input? true)
  :position $ [] 200 120
  :dx 140
  :dy 28
  :action :edit
  :accessibility $ {} (:id |editor) (:role :text-input) (:label "|Document body") (:value |Draft) (:focusable? true)
```

## 6. Deterministic offscreen PNG / 确定性离屏 PNG

This writes only the specified file. Use it for fast CI coverage of visual
scene construction before a native-window smoke test.

它只会写入指定文件。可在原生窗口 smoke 前，用于 CI 中快速覆盖视觉场景构造。

```cirru.no-check
ns cookbook.offscreen $ :require
  calcit-paint.core :refer $ render-to-png!

render-to-png! $ {} (:path |cookbook.png) (:width 160) (:height 90)
  :background $ [] 220 22 14
  :scene $ {} (:type :rectangle)
    :position $ [] 20 20
    :width 120
    :height 50
    :fill-color $ [] 155 70 48
```

## 7. Local image asset / 本地图片资源

Keep assets local and validate the image scene before rendering it. The bundled
fixture is useful for a first check; applications should use their own explicit
relative path and choose sampling deliberately.

资源保持本地，并在绘制前校验 image scene。仓库自带 fixture 适合首次检查；应用应使用
自己的显式相对路径，并明确选择 sampling。

```cirru.no-check
ns cookbook.asset $ :require
  calcit-paint.core :refer $ validate-scene

validate-scene $ {} (:type :image) (:file-path |resources/calcit.png) (:x 20) (:y 20) (:w 96) (:h 56) (:fit :contain) (:sampling :linear)
```

## Agent repair loop / Agent 修复闭环

1. Locate the API with `calcit ./calcit.cirru query defs calcit-paint.core`
   and `query examples <namespace/definition>`.
2. Copy the smallest matching recipe above; do not begin with the integrated
   default scene.
3. Run `validate-scene` before launching a window. An empty list is the only
   success result; retain structural paths verbatim when reporting a failure.
4. Run `calcit ./calcit.cirru --check-only`, then `./scripts/check-cookbook.sh`
   for offscreen/public-API coverage, and finally the native smoke when input,
   focus, or accessibility is involved.
5. When changing `calcit.cirru`, first run `calcit docs agents --full` and use
   structural `calcit edit`, `calcit tree`, or cursor commands.

1. 用 `calcit ./calcit.cirru query defs calcit-paint.core` 与
   `query examples <namespace/definition>` 定位 API。
2. 复制上面最小的匹配 recipe，不要从集成默认场景开始。
3. 启动窗口前先运行 `validate-scene`。只有空列表表示成功；报告失败时应原样保留
   结构路径。
4. 运行 `calcit ./calcit.cirru --check-only`，再运行 `./scripts/check-cookbook.sh`
   覆盖离屏/公开 API；涉及输入、focus 或无障碍时最后运行 native smoke。
5. 修改 `calcit.cirru` 前先运行 `calcit docs agents --full`，并使用结构化的
   `calcit edit`、`calcit tree` 或 cursor 命令。
