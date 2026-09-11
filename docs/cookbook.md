# Calcit Paint cookbook / Calcit Paint 场景手册

This is the smallest executable starting point for each public scene capability.
The default entry is the Creative Art product preview; the retained
`calcit-paint.main/render!` scene is the integration/API gallery. Use this
cookbook when copying one focused pattern into an application or an Agent task.

这是每项公开场景能力的最小可执行起点。默认入口是 Creative Art 产品预览；保留的
`calcit-paint.main/render!` 场景是综合/API gallery。在应用或 Agent 任务中复制一个
聚焦模式时，请使用本手册。

Each recipe uses `cirru.no-run`: `calcit docs check-md` parses and statically
checks its `ns`/definition snippet, but does not execute native FFI or write
files. `./scripts/check-cookbook.sh` remains the runnable public-API check.

每个 recipe 使用 `cirru.no-run`：`calcit docs check-md` 会解析并静态检查其中的
`ns`/定义片段，但不会执行 native FFI 或写入文件。可运行的公开 API 检查仍由
`./scripts/check-cookbook.sh` 提供。

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
| Creative Art workflow / 创意绘制工作流 | `build-art-scene`, `export-art-frame!` | deterministic PNG smoke / 确定性 PNG smoke |
| Basic shapes / 基础图元 | `validate-scene` + `:rectangle` | `validate-scene` returns `[]` / 返回 `[]` |
| Structured repair / 结构化修复 | `validate-scene-structured` | inspect stable path/code/field / 检查稳定 path/code/field |
| Group, transform, clip / group、变换、裁剪 | `:group`, `:translate`, `:clip-rect` | `validate-scene` + offscreen PNG |
| Touch and focus / touch 与 focus | `:touch-area`, `:focus-area` | native demo or Xvfb smoke / 原生或 Xvfb smoke |
| Typed events / 强类型事件 | `launch-canvas-typed!`, `PaintEvent` | exhaustive `match` compiles / 穷尽 `match` 可编译 |
| Accessibility / 无障碍 | `:accessibility` on a touch/focus area | native semantic-tree smoke / 原生语义树 smoke |
| Offscreen snapshots / 离屏快照 | `render-to-png!` | PNG signature and pixel tests / PNG 签名与像素测试 |
| Local assets / 本地资源 | `:image` | `validate-scene` then native/offscreen render / 先校验再原生/离屏绘制 |
| Opt-in control builders / 可选控件构造辅助 | `button`, `touch-container`, `text-input-focus-area`, `icon-label` | `validate-scene` + `check-cookbook.sh` |

Use `calcit ./calcit.cirru query examples calcit-paint.core/validate-scene`
for an API-attached validation recipe. This document is the stable index for
the remaining cookbook recipes, including explicit-write offscreen export.

使用 `calcit ./calcit.cirru query examples calcit-paint.core/validate-scene`
获取附着在 API 上的校验示例。本文档是其余 cookbook recipe（包括显式写入的离屏导出）
的稳定索引。

## 1. Basic shape / 基础图元

Expected result: an empty diagnostic list. / 预期结果：空诊断列表。

```cirru.no-run
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

```cirru.no-run
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

```cirru.no-run
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

Run the retained API gallery and use its nested touch and focus
containers for a real event-loop check. In CI, `CALCIT_PAINT_SMOKE_ONCE=1`
covers the same native entrypoint.

运行保留的 API gallery，使用其中的嵌套 touch/focus 容器进行真实
event-loop 检查。CI 使用 `CALCIT_PAINT_SMOKE_ONCE=1` 覆盖同一原生入口。

## 4. Typed event callback / 强类型事件回调

Use the typed entrypoint for new applications. This minimal recipe shows real
`PaintEvent` cases; use the retained API gallery as the runnable exhaustive protocol
reference.

新应用使用 typed 入口。这个最小 recipe 展示真实的 `PaintEvent` case；保留的 API gallery 是
可运行的穷尽协议参考。

```cirru.no-run
ns cookbook.events $ :require
  calcit-paint.core :refer $ WindowOptions launch-canvas-typed!

launch-canvas-typed! (WindowOptions :title |Cookbook :width 640 :height 420 :min-width 320 :min-height 240 :resizable? true)
  fn (event)
    match event
      (:mouse-down payload)
        println $ :x payload
      (:focus-in payload)
        println $ :focus-id payload
      (:accessibility-action payload)
        println $ :operation payload
      _ $ println event
```

## 5. Explicit accessibility / 显式无障碍语义

Add semantics only to an interactive area. A `:focusable? true` annotation must
be attached to `:focus-area`; a touch-only button must omit it.

只为交互 area 添加语义。`:focusable? true` 标注必须挂在 `:focus-area`；纯 touch
按钮必须省略它。

```cirru.no-run
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

```cirru.no-run
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

## 7. Creative Art scene and export / Creative Art 场景与导出

The preview keeps scene generation pure for a given seed/time pair. Validate
the generated scene before rendering, and export only in response to an explicit
user or build action. The public return is intentionally an open scene-map
boundary because Paint's recursive scene DSL is heterogeneous.

预览针对给定 seed/time 保持场景生成纯净。绘制前校验生成场景，并且只在用户或构建流程
显式请求时导出。公开返回值有意保留为开放 scene-map 边界，因为 Paint 的递归 scene DSL
是异构结构。

```cirru.no-run
ns cookbook.creative-art $ :require
  calcit-paint.core :refer $ validate-scene
  calcit-paint.creative-art :refer $ build-art-scene

let
    no-diagnostics $ []
    scene $ build-art-scene 17 0 |preview false
  assert= no-diagnostics $ validate-scene scene
```

Run `./scripts/check-creative-art.sh` for the executable validation and two
byte-identical fixed-input exports. / 运行 `./scripts/check-creative-art.sh`
可执行场景校验，并验证两次固定输入导出的字节完全一致。

## 8. Local image asset / 本地图片资源

Keep assets local and validate the image scene before rendering it. The bundled
fixture is useful for a first check; applications should use their own explicit
relative path and choose sampling deliberately.

资源保持本地，并在绘制前校验 image scene。仓库自带 fixture 适合首次检查；应用应使用
自己的显式相对路径，并明确选择 sampling。

```cirru.no-run
ns cookbook.asset $ :require
  calcit-paint.core :refer $ validate-scene

validate-scene $ {} (:type :image) (:file-path |resources/calcit.png) (:x 20) (:y 20) (:w 96) (:h 56) (:fit :contain) (:sampling :linear)
```

## 9. Structured diagnostic repair / 结构化诊断修复

Use the typed API when a developer tool or Agent needs to locate and repair a
field without parsing prose. The definition-attached example performs a full
invalid scene → diagnostic → repaired scene → empty diagnostics loop.

开发工具或 Agent 需要在不解析自然语言的情况下定位并修复字段时，使用强类型 API。
definition-attached example 会完整执行“非法 scene → 诊断 → 修复后 scene → 空诊断”闭环。

```cirru.no-run
ns cookbook.structured-diagnostic $ :require
  calcit-paint.core :refer $ validate-scene-structured

validate-scene-structured $ {} (:type :opacity) (:alpha 1.5)
  :children $ []
```

Discover and execute the full repair example after `./build.sh`:

构建 dylib 后，发现并执行完整修复示例：

```bash
calcit ./calcit.cirru query examples calcit-paint.core/validate-scene-structured
calcit ./calcit.cirru analyze check-examples --ns calcit-paint.core --def validate-scene-structured
```

## 10. Opt-in control builders / 可选控件构造辅助

Expected result: an empty diagnostics list. Builders return ordinary scene
maps, so the composed scene validates and renders like any hand-written one.
Optional fields are passed as `(%some value)`.

预期结果：空诊断列表。builder 返回普通 scene map，因此组合场景与手写场景一样可校验、可绘制。
可选字段传值使用 `(%some value)`。

```cirru.no-run
ns cookbook.ui-builders $ :require
  calcit-paint.ui :refer $ button ButtonOptions touch-container TouchContainerOptions text-input-focus-area TextInputOptions icon-label IconLabelOptions demo-scene
  calcit-paint.core :refer $ validate-scene

let
    icon $ icon-label
      IconLabelOptions :label |Asset :x 70 :y 190 :gap $ %some 34
    scene $ {} (:type :group)
      :children $ []
        button $ ButtonOptions :id |save :label |Save :x 120 :y 70 :dx 120 :dy 40 :action (%some :save)
        touch-container $ TouchContainerOptions :id |card :label |Card :role :button :x 120 :y 150 :dx 200 :dy 60 :children
          %some $ [] icon
        text-input-focus-area $ TextInputOptions :id |editor :label |Editor :value |Draft :x 120 :y 260 :dx 220 :dy 40
  validate-scene scene
```

The builders are discoverable through `calcit query examples
calcit-paint.ui/button` and the other four definitions. `demo-scene` composes
all of them; `./scripts/check-cookbook.sh` validates and renders it.

这些 builder 可通过 `calcit query examples calcit-paint.ui/button` 及其余四个定义发现。
`demo-scene` 组合全部 builder；`./scripts/check-cookbook.sh` 会校验并绘制它。

## Agent repair loop / Agent 修复闭环

### English

1. Locate the API with `calcit ./calcit.cirru query defs calcit-paint.core`
   and `query examples <namespace/definition>`.
2. Copy the smallest matching recipe above; do not begin with either the
   Creative Art product preview or the integrated API gallery.
3. Run `validate-scene-structured` before launching a window. An empty list is
   the only success result; use stable path/code/field data to repair failures.
4. Run `calcit ./calcit.cirru --check-only`, then `./scripts/check-cookbook.sh`
   for offscreen/public-API coverage, and finally the native smoke when input,
   focus, or accessibility is involved.
5. When changing `calcit.cirru`, first run `calcit docs agents --full` and use
   structural `calcit edit`, `calcit tree`, or cursor commands.

### 中文

1. 用 `calcit ./calcit.cirru query defs calcit-paint.core` 与
   `query examples <namespace/definition>` 定位 API。
2. 复制上面最小的匹配 recipe，不要从 Creative Art 产品预览或综合 API gallery 开始。
3. 启动窗口前先运行 `validate-scene-structured`。只有空列表表示成功；使用稳定的
   path/code/field 数据修复失败。
4. 运行 `calcit ./calcit.cirru --check-only`，再运行 `./scripts/check-cookbook.sh`
   覆盖离屏/公开 API；涉及输入、focus 或无障碍时最后运行 native smoke。
5. 修改 `calcit.cirru` 前先运行 `calcit docs agents --full`，并使用结构化的
   `calcit edit`、`calcit tree` 或 cursor 命令。
