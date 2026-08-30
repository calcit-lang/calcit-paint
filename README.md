## (Toy)Calcit Paint

> 2D renderer for Calcit.

### Usages / 使用方式

It runs [Calcit](https://github.com/calcit-lang/calcit) and is driven by the canonical `calcit.cirru` Snapshot source.

项目直接运行 [Calcit](https://github.com/calcit-lang/calcit)，并以规范的
`calcit.cirru` Snapshot 作为源码。默认场景包含可直接看到效果的渐变、虚线描边和
混合模式、输入事件、文本排版、离屏导出和静态子树缓存 demo。

```bash
./build.sh
calcit ./calcit.cirru
```

Available APIs:

```cirru.no-check
calcit-paint.core/push-drawing-data! |reset-canvas! nil

calcit-paint.core/push-drawing-data! |render-canvas! shape-data

calcit-paint.core/measure-text! text-options

calcit-paint.core/measure-paragraph! paragraph-options

calcit-paint.core/validate-scene scene-data

calcit-paint.core/render-to-png! offscreen-options

calcit-paint.core/request-frame!

calcit-paint.core/focus! |focus-id

calcit-paint.core/focused? |focus-id

calcit-paint.core/blur!

calcit-paint.core/write-clipboard-text! "|Calcit Paint clipboard / 剪贴板"

calcit-paint.core/read-clipboard-text!

calcit-paint.core/launch-canvas-with-options!

calcit-paint.core/launch-canvas-typed!

calcit-paint.core/set-window-title!

calcit-paint.core/request-window-size!

calcit-paint.core/close-window!

calcit-paint.core/launch-canvas! $ fn (event) (println "|rendering to canvas...") (&unit)
```

### Native FFI / 原生 FFI

The C-safe buffer-v1 and blocking-callback descriptors, ownership rules, Cirru
EDN transport, and adapters are provided by
[`calcit_native_ffi`](https://github.com/calcit-lang/calcit-native-ffi). Paint
keeps ownership of the Skia/winit event loop, rendering state, shape decoding,
and callback scheduling. The module tracks `calcit_native_ffi 0.1.2`; buffer
and blocking-callback protocols remain at v1. It requires Calcit 0.13.64.

C-safe buffer-v1/blocking-callback descriptor、ownership 规则、Cirru EDN
transport 与 adapter 由
[`calcit_native_ffi`](https://github.com/calcit-lang/calcit-native-ffi) 统一维护。
Paint 仍负责 Skia/winit 事件循环、绘制状态、shape 解析和回调调度；
模块当前使用 `calcit_native_ffi 0.1.2`，buffer 与 blocking-callback protocol
均继续保持 v1；模块要求 Calcit 0.13.64。

A custom mirror may be used for Skia binaries when it is known to be healthy:

```bash
export SKIA_BINARIES_URL=https://cdn.tiye.me/skia-binaries/{tag}/skia-binaries-{key}.tar.gz
```

If the mirror is unavailable, unset `SKIA_BINARIES_URL` to use rust-skia's
official GitHub release assets. CI deliberately leaves this variable unset.

只在镜像站可用时设置 `SKIA_BINARIES_URL`；镜像异常时取消该变量，
回退到 rust-skia 的 GitHub Release 产物。CI 不设置该变量。

### Shapes

Position represented with `[] x y`. Color with `[] h s l a?`.
位置使用 `[] x y`，颜色使用 `[] h s l a?`。

Drawing is backed by Skia and uses logical pixels. Shape maps support the
following primitives and containers:

绘制由 Skia 完成，坐标单位为逻辑像素。shape map 支持下列图元与容器。

#### Paint, gradients, and strokes / Paint、渐变与描边

`rectangle`, `rounded-rect`, `circle`, `ellipse`, `arc`, `ops`, and
`touch-area` accept a `:fill` paint map and a `:stroke` style map. `polyline`
accepts `:stroke`. Solid paint, linear gradients, and radial gradients share
the same paint schema, so gradients can be used for either fill or stroke.

`rectangle`、`rounded-rect`、`circle`、`ellipse`、`arc`、`ops` 和
`touch-area` 可使用 `:fill` paint map 与 `:stroke` style map；`polyline`
可使用 `:stroke`。纯色、线性渐变和径向渐变共用同一套 paint schema，因此填充和
描边都可以使用渐变。

```cirru
{} (:type :rounded-rect)
  :position $ [] 40 40
  :width 220
  :height 80
  :radius 16
  :fill $ {} (:type :linear-gradient)
    :from $ [] 40 40
    :to $ [] 260 120
    :stops $ []
      [] 0 $ [] 16 90 60
      [] 0.5 $ [] 330 85 62
      [] 1 $ [] 210 85 55
```

```cirru
{} (:type :circle)
  :position $ [] 360 90
  :radius 55
  :fill $ {} (:type :radial-gradient)
    :center $ [] 342 72
    :radius 72
    :stops $ []
      [] 0 $ [] 52 95 72
      [] 0.55 $ [] 18 90 58
      [] 1 $ [] 348 85 42
```

Use `:type :solid` when a structured solid paint is useful. Stroke options are
`:width`, `:cap` (`:butt`, `:round`, `:square`), `:join` (`:miter`, `:round`,
`:bevel`), `:miter-limit`, `:dash`, and `:dash-offset`.

需要结构化纯色 paint 时使用 `:type :solid`。描边可配置 `:width`、`:cap`
（`:butt`、`:round`、`:square`）、`:join`（`:miter`、`:round`、`:bevel`）、
`:miter-limit`、`:dash` 和 `:dash-offset`。

```cirru
{} (:type :rectangle)
  :position $ [] 40 160
  :width 220
  :height 70
  :stroke $ {}
    :paint $ {} (:type :solid)
      :color $ [] 192 90 68
    :width 5
    :cap :round
    :join :miter
    :miter-limit 6
    :dash $ [] 14 8
    :dash-offset 3
```

Gradient stops must contain at least two entries, use offsets in `[0, 1]`, and
be strictly increasing. Linear-gradient endpoints must differ; radial radius
and every dash interval must be positive. A dash list must be non-empty and
contain an even number of intervals. Invalid values are rejected with a field-
specific error.

渐变至少需要两个 stop，offset 必须位于 `[0, 1]` 且严格递增；线性渐变的两个端点
不能相同，径向渐变半径与每段 dash 长度必须为正数；dash 列表不能为空且长度必须为
偶数。非法值会返回指向具体字段的错误。

Legacy `:fill-color`, `:line-color`, and `:line-width` remain supported. The
legacy `polyline` fields `:color`, `:width`, `:cap`, and `:join` also remain
supported. Do not mix a legacy form with its structured replacement on the same
shape.

旧版 `:fill-color`、`:line-color`、`:line-width` 继续兼容；`polyline` 的
`:color`、`:width`、`:cap`、`:join` 也继续兼容。同一个 shape 上不要混用旧字段与
对应的新结构。

Gradient shaders and dash path effects are cached by their complete immutable
descriptions. Reusing the same description reuses the Skia object; changing any
coordinate, stop, color, radius, interval, or offset produces a different cache
key. Each cache is capped at 256 entries and is cleared when that bound is
reached.

渐变 shader 与 dash path effect 会按完整、不可变的描述缓存。描述相同就复用 Skia
对象；任意坐标、stop、颜色、半径、interval 或 offset 改变都会产生新的缓存 key。
每个缓存最多保留 256 项，到达上限后整体清空。

#### Blend layers / 混合图层

Use a `blend` container to composite all of its children as one Skia layer
against the existing backdrop. Supported modes are `:src-over`, `:multiply`,
`:screen`, `:overlay`, `:darken`, `:lighten`, `:difference`, `:exclusion`, and
`:plus`.

使用 `blend` 容器可将所有 children 作为一个 Skia 图层，与已有背景进行合成。支持
`:src-over`、`:multiply`、`:screen`、`:overlay`、`:darken`、`:lighten`、
`:difference`、`:exclusion` 和 `:plus`。

```cirru
{} (:type :blend) (:mode :multiply)
  :children $ []
    {} (:type :circle)
      :position $ [] 360 210
      :radius 48
      :fill-color $ [] 215 90 60
```

- Rect, using `rect` or `rectangle`:

```rust
rect {
  position: Vec2,
  width: f32,
  height: f32,
  line_style: Option<StrokeStyle>,
  fill_style: Option<PaintSource>,
},
```

- Group, using `group`

```rust
Group {
  position: Vec2,
  children: Vec<Shape>,
},
```

- Circle, using `circle`

```rust
Circle {
  position: Vec2,
  radius: f32,
  line_style: Option<StrokeStyle>,
  fill_style: Option<PaintSource>,
},
```

- Rounded rect, using `rounded-rect` or `rounded-rectangle`. Use `radius` for
  both axes, or `radius-x` and `radius-y` independently:

```cirru
{} (:type :rounded-rect)
  :position $ [] 20 20
  :width 120
  :height 60
  :radius 16
  :fill-color $ [] 210 70 55
```

- Ellipse, using `ellipse`, with `radius-x` and `radius-y`.

- Arc, using `arc`. Angles are in degrees; positive sweep is clockwise.
  `use-center? true` draws a wedge:

```cirru
{} (:type :arc)
  :position $ [] 200 120
  :radius-x 80
  :radius-y 40
  :start-angle 190
  :sweep-angle 160
  :use-center? false
  :line-color $ [] 120 80 70
  :line-width 4
```

#### Text layout / 文本排版

`text` accepts the required `:text`, `:position`, `:size`, `:color`, and
`:align` fields, plus the following optional text-layout fields. The default
behaviour is compatible with existing text shapes: a 400-weight, upright,
platform-default font is drawn with `:alphabetic` baseline.

`text` 需要 `:text`、`:position`、`:size`、`:color` 和 `:align`；还支持以下可选
排版字段。缺省行为与旧 text shape 兼容：使用平台默认字体、400 字重、常规样式，并以
`:alphabetic` 基线绘制。

| Field / 字段 | Values / 取值 | Default / 默认值 |
| --- | --- | --- |
| `:font-family` | Font-family string / 字体族字符串 | System default / 系统默认字体 |
| `:weight` | Integer from `100` to `900` / `100` 至 `900` 的整数 | `400` |
| `:style` | `:normal`, `:italic` | `:normal` |
| `:baseline` | `:alphabetic`, `:top`, `:middle`, `:bottom` | `:alphabetic` |
| `:align` | `:left`, `:center`, `:right` | Required / 必填 |

```cirru.no-check
{} (:type :text) (:text "|Bold italic · top")
  :position $ [] 530 110
  :color $ [] 42 90 92
  :size 24
  :font-family |monospace
  :weight 700
  :style :italic
  :baseline :top
  :align :left
```

`position` is the selected alignment anchor on the selected baseline. Thus
`:top`, `:middle`, and `:bottom` keep those respective visual locations stable;
`:alphabetic` preserves Skia's traditional text origin. A requested font family
that is not installed is not an error: Skia falls back to the platform default
while retaining the requested weight and style as closely as available.

`position` 是所选对齐方式和基线的锚点。`:top`、`:middle`、`:bottom` 会稳定对应的
视觉位置；`:alphabetic` 则保持 Skia 传统的文字原点。若请求的字体族未安装，不会报错：
Skia 会回退到平台默认字体，并尽可能保留请求的字重和样式。

Weights must be integral values in the inclusive `100..900` range; unknown
styles or baselines are rejected with field-specific errors. For compatibility,
legacy numeric string weights such as `|300` are also accepted, but new code
should use numbers.

字重必须是 `100..900`（含边界）范围的整数；未知样式或基线会返回指向字段的错误。为
兼容旧代码，`|300` 这样的数字字符串字重仍可用；新代码应使用数字。

`calcit-paint.core/measure-text!` measures text without drawing it. It accepts a
map with `:text`, `:size`, and the same optional font fields above, and returns
an EDN map with `:width`, `:height`, `:line-height`, `:ascent`, `:descent`,
`:leading`, and `:baseline`. `:baseline` is the distance from the line box top
to the alphabetic baseline; an empty string has zero width and retains its font
line metrics.

`calcit-paint.core/measure-text!` 可在不绘制时测量文本。它接收包含 `:text`、`:size`
及上述可选字体字段的 map，返回带有 `:width`、`:height`、`:line-height`、`:ascent`、
`:descent`、`:leading` 和 `:baseline` 的 EDN map。`:baseline` 表示从行框顶部到
alphabetic 基线的距离；空字符串宽度为零，仍保留对应字体的行度量。

```cirru.no-check
measure-text! $ {} (:text "|Text layout / 文本排版") (:size 24) (:font-family |monospace) (:weight 700) (:style :italic) (:baseline :middle)
```

Run `./build.sh` followed by `calcit ./calcit.cirru` to run the maintained
Calcit demo. It prints the measurement map before opening the canvas, then
displays bold italic/top, regular/middle, and light/bottom text samples.

执行 `./build.sh` 后运行 `calcit ./calcit.cirru` 即可启动维护中的 Calcit demo。
它会在打开画布前打印测量 map，并显示粗斜体/top、常规/middle、细体/bottom 三组文本。

#### Paragraph layout and international text / 段落布局与国际化文本

Use `:paragraph` (or the `:text-block` alias) for wrapping and multi-line text.
It uses Skia Paragraph/TextLayout with ICU/BiDi shaping; it never slices UTF-8
bytes as a fallback. The existing single-line `:text` shape and
`measure-text!` API remain unchanged.

使用 `:paragraph`（或别名 `:text-block`）绘制自动换行和多行文本。实现直接使用带
ICU/BiDi shaping 的 Skia Paragraph/TextLayout，不会通过切分 UTF-8 字节来降级处理。
现有单行 `:text` shape 与 `measure-text!` API 保持不变。

```cirru.no-check
{} (:type :paragraph) (:text "|Calcit Paint paragraph\n中文段落 · explicit newline")
  :position $ [] 40 610
  :max-width 300
  :color $ [] 42 90 92
  :size 20
  :line-height 28
  :align :left
  :direction :ltr
  :max-lines 2
  :ellipsis "|…"
```

`position` is the top-left corner of the paragraph layout box. `:max-width`
and `:size` must be finite positive numbers. `:line-height`, when present, is
an absolute logical-pixel height and must also be positive. `:align` accepts
`:left`, `:center`, or `:right` and defaults to `:left`; `:direction` accepts
`:ltr` or `:rtl` and defaults to `:ltr`. `:max-lines` must be a positive
integer. A string `:ellipsis` requires `:max-lines`, which prevents a silently
ineffective truncation option. Font family, weight, style, and color behave the
same as on single-line text.

`position` 表示段落布局框的左上角。`:max-width` 与 `:size` 必须是有限正数；可选的
`:line-height` 是以逻辑像素表示的绝对行高，也必须为正数。`:align` 支持 `:left`、
`:center`、`:right`，默认 `:left`；`:direction` 支持 `:ltr`、`:rtl`，默认 `:ltr`。
`:max-lines` 必须为正整数；字符串 `:ellipsis` 必须与 `:max-lines` 一起使用，避免配置
省略号却不生效。字体族、字重、样式和颜色沿用单行文本语义。

`measure-paragraph!` accepts the layout fields above (drawing-only
`:position`, `:color`, and `:type` are unnecessary) and returns a homogeneous
`Map<Tag, Number>` containing `:width`, `:height`, `:line-count`, `:max-width`,
`:min-intrinsic-width`, `:max-intrinsic-width`, `:alphabetic-baseline`, and
`:ideographic-baseline`. Drawing and measurement use the same layout helper.

`measure-paragraph!` 接收上述布局字段（不需要仅绘制使用的 `:position`、`:color`、
`:type`），返回同构的 `Map<Tag, Number>`，字段包括 `:width`、`:height`、
`:line-count`、`:max-width`、`:min-intrinsic-width`、`:max-intrinsic-width`、
`:alphabetic-baseline` 与 `:ideographic-baseline`。绘制与测量共用同一布局实现。

The maintained default Calcit demo prints paragraph metrics and renders three
real paragraph shapes: Chinese/English with an explicit newline, constrained
two-line ellipsis, and right-to-left Arabic. Run it with the commands at the
top of this README.

维护中的默认 Calcit demo 会打印段落度量，并实际绘制三组 paragraph shape：含显式
换行的中英文、受限为两行并带省略号的段落，以及从右向左的阿拉伯文。使用 README
开头的命令即可运行。

- Paint operations, with `ops`

```rust
PaintOps {
  position: Vec2,
  path: Vec<PaintPathTo>,
  line_style: Option<StrokeStyle>,
  fill_style: Option<PaintSource>,
},
```

Paths use Skia's path builder with this compact operation subset:

```cirru
[]
  [] :move-to $ [] 1 2
  [] :line-to $ [] 3 4
  [] :quadratic-bezier-to ([] 5 6) ([] 7 8)
  [] :cubic-bezier-to ([] 1 2) ([] 3 4) ([] 5 6)
  [] :close-path
```

The shorter aliases are also supported:
`bezier2-to` -> `quadratic-bezier-to`, and
`bezier3-to` -> `cubic-bezier-to`.

- Polyline, using `polyline`

```rust
Polyline {
  position: Vec2,
  stops: Vec<Vec2>,
  skip_first: bool,
  line_style: StrokeStyle,
},
```

- Image presentation / 图片展示，使用 `image` 与 Skia `Rect`

```rust
Image {
  file_path: String,
  x: f32, y: f32, w: f32, h: f32,
  crop: Option<Rect>, // Calcit map fields: :x, :y, :w, :h
  fit: ImageFit,
  sampling: ImageSampling,
}
```

The destination `:x` and `:y` must be finite; `:w` and `:h` must be finite and
positive. `:fit` is an optional tag:

| `:fit` | English | 中文 |
| --- | --- | --- |
| `:fill` (default) | Stretch to the destination rectangle; preserves the legacy behavior. | 拉伸到目标矩形；保持旧版本行为。 |
| `:contain` | Preserve aspect ratio and center the complete image inside the destination. | 保持宽高比，在目标矩形内居中显示完整图片。 |
| `:cover` | Preserve aspect ratio, fill the destination, and crop the source from the center. | 保持宽高比并填满目标矩形，从源图中心裁切。 |

`:sampling` is also optional. `:nearest` is the compatibility-preserving
default, `:linear` uses linear filtering, and `:cubic` uses Skia's Mitchell
cubic resampler. / `:sampling` 同样可省略；兼容默认值为 `:nearest`，
`:linear` 使用线性过滤，`:cubic` 使用 Skia 的 Mitchell 三次重采样。

`:crop` may be omitted or set to `nil`. When present it must be a map with
finite non-negative `:x` / `:y` and finite positive `:w` / `:h`. Scene
validation checks this structure without doing file I/O; after decoding, the
renderer also verifies that the crop stays inside the actual image and reports
the structural shape path on failure. / `:crop` 可以省略或设为 `nil`；存在时
必须是 map，其中 `:x` / `:y` 为有限非负数，`:w` / `:h` 为有限正数。
场景校验不读取文件，解码后渲染器会继续检查裁切范围是否位于真实图片内，
错误信息会包含结构化 shape 路径。

Decoded images use an LRU cache capped at 64 entries and 64 MiB of estimated
RGBA memory (`width * height * 4`). File-size or modification-time changes
invalidate an entry; an oversized image is still rendered but is not cached.
Missing files keep the legacy log-and-skip behavior. / 解码图片使用 LRU 缓存，
上限为 64 项及 64 MiB 预估 RGBA 内存（`width * height * 4`）。文件大小或
修改时间变化会使缓存失效；超限单图仍可渲染，但不会进入缓存。文件缺失时
保持原有的记录错误并跳过渲染行为。

The runnable `calcit-paint.main/render!` demo exercises legacy fill/nearest,
contain/linear, and cropped cover/cubic in one scene. / 可运行的
`calcit-paint.main/render!` demo 在同一场景中覆盖旧版 fill/nearest、
contain/linear，以及带 crop 的 cover/cubic。

- Cached group, using `cached-group` (alias: `static-group`)

```rust
CachedGroup {
  cache_key: String,
  revision: i32,
  position: Vec2,
  width: i32,
  height: i32,
  children: Vec<Shape>,
}
```

- Hierarchical clips / 层级裁剪

`clip-rect` clips all descendants to rectangular bounds. `clip-rounded-rect`
(alias: `clip-rounded-rectangle`) uses the same `:radius` or independent
`:radius-x` / `:radius-y` fields as `rounded-rect`. Width, height, and radii
must be finite non-negative numbers. Radii larger than half the corresponding
dimension are clamped consistently by Skia painting and Paint hit testing; a
zero width or height produces an empty visual and interactive clip.

`clip-rect` 将所有后代裁剪到矩形边界；`clip-rounded-rect`（别名
`clip-rounded-rectangle`）使用与 `rounded-rect` 相同的 `:radius`，或分别使用
`:radius-x` / `:radius-y`。宽、高和半径必须是有限非负数；超过对应尺寸一半的半径会
在 Skia 绘制与 Paint 命中测试中一致地收窄；宽或高为零时，视觉与交互 clip 均为空。

```cirru.no-check
{} (:type :clip-rounded-rect) (:radius 20)
  :position $ [] 220 80
  :width 260
  :height 160
  :children $ []
    {} (:type :translate) (:x 30) (:y 0)
      :children $ []
        {} (:type :touch-area) (:dx 65) (:dy 18) (:cursor :pointer)
          :position $ [] 415 210
          :action :clipped-target
```

Clips are hierarchical intersections and retain the transform active at each
clip node. The same nested clip stack constrains Skia painting, hover/cursor
selection, pointer down, and pointer-triggered focus. This prevents invisible
content outside a scroll panel or rounded card from remaining interactive.
Singular clip transforms reject hits safely. Keyboard/programmatic focus remains
available because clipping affects pointer hit testing, not logical tab order.

裁剪按层级取交集，并保留每个 clip 节点处生效的 transform。同一组嵌套 clip stack
同时约束 Skia 绘制、hover/cursor、pointer down 与指针触发的 focus，避免滚动面板或
圆角卡片之外的不可见内容仍可交互。退化且不可逆的 clip transform 会安全地拒绝命中。
键盘与程序化 focus 仍然可用，因为裁剪只影响指针命中，不改变逻辑 tab 顺序。

- Opacity, using `opacity`, composites all children as one layer. `alpha` must
  be between `0` and `1`:

```cirru
{} (:type :opacity) (:alpha 0.6)
  :children $ []
    {} (:type :circle)
      :position $ [] 80 80
      :radius 40
      :fill-color $ [] 20 80 60
```

- Touch Area, using `touch-area`

For handling events:

```rust
TouchArea {
  target: EventTarget {
    action: Option<Calcit>,
    path: Option<Calcit>,
    data: Option<Calcit>,
  },
  position: Vec2,
  // children: Vec<Shape>, // TODO
  area: TouchAreaShape,
  cursor: Option<CursorIcon>,
  line_style: Option<StrokeStyle>,
  fill_style: Option<PaintSource>,
},
```

- Key listener, using `key-listener`

```rust
KeyListener {
  key: String,
  modifiers: Option<ShortcutModifiers>,
  focus_id: Option<String>,
  target: EventTarget {
    action: Option<Calcit>,
    path: Option<Calcit>,
    data: Option<Calcit>,
  },
  // children: Vec<Shape>, // TODO
},
```

- Focus area, using `focus-area` (alias: `focusable`)

```rust
FocusArea {
  id: String,
  target: EventTarget,
  position: Vec2,
  area: TouchAreaShape,
  tab_index: i32,
  text_input: bool,
  line_style: Option<StrokeStyle>,
  fill_style: Option<PaintSource>,
}
```

`:action`, `:path`, and `:data` are optional on touch-area, key-listener, and
focus-area shape maps. Omitting a
field and passing an explicit `nil` are equivalent. Internally Paint stores
them as `Option<Edn>`; when a touch area or key listener matches, the emitted
event still contains all three legacy keys and uses `nil` for absent values.
This keeps existing event consumers compatible while allowing new Calcit demos
to omit meaningless placeholders.

`:action`、`:path` 与 `:data` 在 touch-area、key-listener、focus-area 三类 shape map
中均为可选字段；省略字段与显式传入
`nil` 等价。Paint 内部使用 `Option<Edn>` 存储；当 touch area 或 key listener
命中时，发出的事件仍保留这三个历史字段，缺失值以 `nil` 表示。因此旧事件消费代码
保持兼容，而新的 Calcit demo 不再需要填写无意义的占位值。

`touch-area` also accepts optional `:cursor` as a strict tag. Missing or `nil`
uses the platform default; strings and unknown tags are rejected by scene
validation. Paint accepts the W3C cursor names supported by winit, including
`:default`, `:pointer`, `:text`, `:crosshair`, `:grab`, `:grabbing`, `:move`,
`:not-allowed`, the directional resize tags, `:col-resize`, `:row-resize`,
`:all-scroll`, `:zoom-in`, and `:zoom-out`.

`touch-area` 还可使用严格 tag 类型的可选 `:cursor`。省略或传入 `nil` 时使用平台
默认光标；字符串和未知 tag 会被 scene validation 拒绝。Paint 接受 winit 支持的
W3C cursor 名称，包括 `:default`、`:pointer`、`:text`、`:crosshair`、`:grab`、
`:grabbing`、`:move`、`:not-allowed`、各方向 resize tag、`:col-resize`、
`:row-resize`、`:all-scroll`、`:zoom-in` 与 `:zoom-out`。

```cirru.no-check
{} (:type :touch-area) (:dx 80) (:dy 30) (:cursor :grab)
  :position $ [] 200 120
  :action :drag-card
  :path $ [] :card |demo
```


### Offscreen rendering and snapshots / 离屏渲染与快照

`render-to-png!` renders the same shape maps through a CPU raster surface and
writes one PNG to the explicit `:path`. Nothing is written unless this function
is called. The destination parent must already exist, and an existing file at
that exact path is replaced. `:scene` is required (`:shape` is an alias), while
`:background` defaults to transparent.

`render-to-png!` 使用同一套 shape map 和 CPU raster surface 绘制，并只向显式
`:path` 写入一个 PNG；没有调用该函数就不会写文件。目标父目录必须已经存在，指定路径
上的旧文件会被替换。必须提供 `:scene`（`:shape` 是兼容别名），`:background` 默认透明。

```cirru.no-check
render-to-png! $ {} (:path |snapshot.png) (:width 320) (:height 180)
  :background $ [] 225 25 12
  :scene $ {} (:type :group)
    :children $ []
      {} (:type :rectangle)
        :position $ [] 20 20
        :width 120
        :height 80
        :fill-color $ [] 205 70 42
```

Width and height are integer logical pixels from `1` through `4096`, with a
total limit of `16,777,216` pixels. One logical pixel maps to one raster pixel.
The surface contract is CPU-backed RGBA8888, premultiplied alpha, and sRGB;
the window GPU surface is not part of the snapshot contract. Offscreen drawing
does not register touch, key, or focus targets. Geometry-only scenes are stable
enough for exact RGBA key-pixel and full pixel-hash assertions. PNG encoder
bytes, system font fallback, text shaping, and decoded external images may vary
across Skia/platform versions, so CI snapshots keep those out of cross-platform
exact hashes or maintain platform-specific baselines.

宽高必须是 `1` 到 `4096` 的整数逻辑像素，总像素数不超过 `16,777,216`；一个逻辑
像素对应一个 raster 像素。快照契约固定为 CPU-backed RGBA8888、预乘 alpha 与 sRGB，
不包含窗口 GPU surface。离屏绘制不会注册 touch、key 或 focus target。纯几何 scene
可以断言精确关键 RGBA 像素和完整 pixel hash；PNG encoder bytes、系统字体 fallback、
文字 shaping 及外部图片解码可能随 Skia/平台版本变化，因此跨平台 CI 的精确 hash 应避开
这些内容，或使用分平台 baseline。

`cached-group` is an optional CPU-raster prototype for static children. Its
children use local coordinates and the resulting image is placed at
`:position`. A cache hit requires the same `:cache-key`, `:revision`, `:width`,
and `:height`; callers must increment `:revision` whenever children or dependent
resources change. The process-wide LRU is bounded to 32 entries and 32 MiB
(`width × height × 4` bytes per entry). Interactive descendants are rejected
instead of silently losing events. This explicit invalidation model avoids
walking and hashing arbitrary heterogeneous EDN on every frame.

`cached-group` 是面向静态 children 的可选 CPU raster 原型；children 使用局部坐标，
缓存图片再放置到 `:position`。只有 `:cache-key`、`:revision`、`:width`、`:height`
全部相同才会命中；children 或依赖资源变化时，调用方必须递增 `:revision`。进程级 LRU
上限为 32 个 entry、32 MiB（每项占 `width × height × 4` 字节）。交互子节点会直接报错，
不会静默丢失事件。显式 revision 避免每帧遍历并 hash 任意异构 EDN。

The default scene shows a cached badge. Press Shift+P to explicitly run
`export-offscreen-demo!` and create `offscreen-demo.png` in the current working
directory; startup itself does not write the file.

默认 scene 会显示 cached badge；按 Shift+P 会显式调用 `export-offscreen-demo!`，在当前
工作目录生成 `offscreen-demo.png`，程序启动本身不会写出该文件。

### Scene validation and diagnostics / 场景校验与诊断

`validate-scene` runs the same strict shape decoder used by windowed and
offscreen rendering. It returns `List<String>`: an empty list means the scene is
valid; every invalid sibling contributes one stable, structural-path diagnostic
such as `$.children[1].children[0]: expected a map, got true`. `nil` remains the
compatible empty scene. Valid scenes keep their existing rendering behavior.

`validate-scene` 使用窗口渲染与离屏渲染共用的严格 shape decoder，返回
`List<String>`：空列表表示 scene 合法；同级非法节点会分别产生稳定、可定位的结构路径
诊断，例如 `$.children[1].children[0]: expected a map, got true`。`nil` 继续作为兼容的
空 scene，合法 scene 的既有渲染行为不变。

```cirru.no-check
let
    scene $ {} (:type :group)
      :children $ []
        {} (:type :rounded-rect) (:width 160) (:height 70) (:radius 12)
    diagnostics $ validate-scene scene
  println diagnostics
```

Invalid nested shapes are no longer replaced with empty groups. Windowed
rendering reports the strict failure on stderr, while `render-to-png!` returns
the same diagnostic and does not write a partial PNG. Renderer diagnostics and
unknown drawing operations never print to stdout. The default Calcit entry runs
`validate-scene-demo!` before opening the window and prints both a passing scene
and two expected nested failures, so the API is exercised by the normal demo.

非法嵌套 shape 不再被静默替换为空 group。窗口渲染会将严格校验失败写到 stderr；
`render-to-png!` 返回相同诊断，且不会写出不完整 PNG。渲染诊断与未知绘制操作均不再
污染 stdout。默认 Calcit 入口会在打开窗口前实际运行 `validate-scene-demo!`，打印一个
通过的 scene 和两个预期的嵌套错误，因此正常 demo 会真实覆盖该 API。

### On-demand frames and animation timing / 按需帧与动画时钟

`request-frame!` schedules one `:frame` callback on the active Paint window. It
is intentionally one-shot: repeated requests before delivery are coalesced, and
continuous animation must request its next frame from the callback. Paint does
not switch the event loop to permanent polling, so an idle scene remains idle.
Calling it without an active blocking `launch-canvas!` callback returns an
error.

`request-frame!` 会在当前 Paint 窗口上调度一次 `:frame` callback。它是有意设计的
one-shot API：事件送达前的重复请求会合并；连续动画必须在 callback 内显式请求下一帧。
Paint 不会把事件循环切换成永久轮询，因此静止场景不会持续占用资源。没有活跃的
blocking `launch-canvas!` callback 时调用会返回错误。

```cirru.no-check
launch-canvas-typed! options $ fn (event)
  match event
    (:frame payload)
      do
        render-animation! $ :timestamp-ms payload
        request-frame!
    _ $ &unit
```

A frame event contains `:frame`, monotonic `:timestamp-ms`, `:delta-ms`, logical
`:width` and `:height`, and `:scale-factor`. The first delivered frame and the
first frame after an occluded, minimized, or suspended interval use a zero
delta. While paused, Paint retains at most one pending request and delivers it
after restoration; closing the window cancels it.

帧事件包含 `:frame` 序号、单调递增的 `:timestamp-ms`、`:delta-ms`、逻辑像素
`:width`/`:height` 与 `:scale-factor`。首帧以及窗口从遮挡、最小化或 suspended
状态恢复后的第一帧，其 delta 为零。暂停期间至多保留一个待处理请求，并在恢复后送达；
窗口关闭时请求会被取消。

The bundled runnable demo requests one startup frame and then stays idle. Press
`A` to start or pause its animated circle, demonstrating explicit frame
chaining without a busy loop.

随仓库提供的可运行 demo 会在启动时请求一帧，随后保持空闲。按 `A` 可开始或暂停圆形
动画，用于演示不依赖 busy loop 的显式逐帧调度。

### Window lifecycle / 窗口生命周期

`launch-canvas!` remains compatible: it opens one resizable 1100×760 logical-pixel
window titled `Calcit Paint`, without an explicit minimum size. New applications
can pass the nominal `WindowOptions` value to `launch-canvas-typed!`; the
compatible `launch-canvas-with-options!` entry remains available for map-based
callbacks.
Every field is required and checked at the native boundary: dimensions must be
finite and positive, minimum dimensions cannot exceed the initial dimensions,
and maps or unrelated structs are rejected. Paint intentionally remains a
single-window module; a second launch fails explicitly.

`launch-canvas!` 保持兼容：它仍打开一个标题为 `Calcit Paint`、逻辑尺寸为
1100×760、可调整大小且没有显式最小尺寸的单窗口。新应用可以把 nominal
`WindowOptions` 传给 `launch-canvas-typed!`；兼容入口
`launch-canvas-with-options!` 继续提供 map callback。所有字段都必须提供，并会在
native 边界严格校验：尺寸必须是有限正数，最小尺寸不得超过初始尺寸，map 或其他
struct 会被拒绝。Paint 仍有意保持单窗口模型，重复启动会明确报错。

```cirru.no-check
launch-canvas-typed!
  WindowOptions (:title "|My Paint window") (:width 1100) (:height 760) (:min-width 720) (:min-height 520) (:resizable? true)
  fn (event)
    match event
      (:key-down payload)
        case-default (:name payload) (println payload)
          |T $ set-window-title! "|Updated title"
          |S $ request-window-size! 980 700
          |Q $ close-window!
      _ $ println event
```

Runtime requests are valid only while a launch callback is active. They enter a
FIFO queue and are applied on the event loop after the current callback returns,
so a request never re-enters that callback. A successful `Unit` return means
“queued”, not “accepted by the platform”. Title updates produce an applied
`:window-request` event. A size request produces either `:status :confirmed`
with logical `:actual-width`, `:actual-height`, and `:matched?`, or
`:status :pending` with those acknowledgement fields set to `nil`; a subsequent
`:resize` event is authoritative. `:matched? false` reports a clamped or denied
platform result. Resize events now also include `:scale-factor`, and display
scale changes emit `:type :scale-factor` with the current logical size.

运行期请求仅能在 launch callback 活跃期间调用。请求进入 FIFO 队列，并在当前
callback 返回后由 event loop 串行应用，因此不会重入 callback。成功返回 `Unit` 只表示
“已入队”，不表示平台已经接受。标题更新会产生 applied 的 `:window-request` 事件。
尺寸请求会产生两类确认：`:status :confirmed` 携带逻辑尺寸
`:actual-width`、`:actual-height` 和 `:matched?`；`:status :pending` 时这些确认字段为
`nil`，后续 `:resize` 才是权威结果。`:matched? false` 表示平台进行了限制或拒绝。
`:resize` 现在也包含 `:scale-factor`，显示缩放变化会发送包含当前逻辑尺寸的
`:type :scale-factor` 事件。

The typed callback maps these acknowledgements to distinct
`:window-title-applied` and `:window-size-request` variants, so application code
does not need to branch on a second `:operation` tag. The legacy callback shape
is unchanged.

强类型 callback 会把两类确认分别映射为 `:window-title-applied` 与
`:window-size-request` variant，应用不必再对第二层 `:operation` tag 分支；旧 callback
结构保持不变。

Closing emits exactly one `{:type :window-close :reason ...}` event before the
event loop returns. Reasons are `:requested`, `:system`, `:escape`,
`:render-error`, `:smoke`, or the defensive `:event-loop` fallback. The bundled
runnable demo uses configured startup options; press `T` to change its title,
`S` to request 980×700, and `Q` to close it safely.

关闭前会且仅会发送一次 `{:type :window-close :reason ...}` 事件，随后 event loop
返回。reason 可能为 `:requested`、`:system`、`:escape`、`:render-error`、`:smoke`
或兜底的 `:event-loop`。仓库内可运行 demo 已使用启动配置；按 `T` 修改标题，按 `S`
请求 980×700，按 `Q` 安全关闭。

### Nominal PaintEvent protocol / Nominal PaintEvent 协议

`launch-canvas-typed!` is the preferred entry for new Calcit applications. Its
callback receives the closed `PaintEvent` enum instead of legacy `nil` and
heterogeneous maps. Startup is `(:ready)`; every payload-bearing variant uses a
nominal struct, and `match` checks variant names, payload arity, and exhaustiveness.
The bundled runnable demo uses this entry and matches all 30 variants.

新 Calcit 应用优先使用 `launch-canvas-typed!`。callback 接收封闭的 `PaintEvent`
enum，不再接收旧版 `nil` 与异构 map。启动事件为 `(:ready)`；所有带 payload 的 variant
都使用 nominal struct，`match` 会检查 variant 名称、payload 数量与穷尽性。仓库内可运行
demo 已切换到该入口，并完整匹配全部 30 个 variant。

Payloads are grouped by domain: `PaintPointerEvent`, `PaintKeyboardEvent`,
`PaintFocusEvent`, `PaintTextInputEvent`, `PaintFileEvent`, `PaintFrameEvent`,
and the window payload structs. Optional protocol fields use `Option<T>` rather than `nil`.
Application-defined `:action`, `:path`, and `:data` are intentionally isolated
inside `PaintTarget` as `Option<Dynamic>`; this is the only open application
payload in the public event model.

Payload 按领域拆分为 `PaintPointerEvent`、`PaintKeyboardEvent`、`PaintFocusEvent`、
`PaintTextInputEvent`、`PaintFileEvent`、`PaintFrameEvent` 以及各类 window payload
struct。协议中的
可选字段使用 `Option<T>`，不再使用 `nil`。应用自定义的 `:action`、`:path`、`:data`
被集中隔离在 `PaintTarget` 中，以 `Option<Dynamic>` 表达；这是公开事件模型唯一开放的
应用 payload。

```cirru.no-check
launch-canvas-typed! options $ fn (event)
  match event
    (:ready) (request-frame!)
    (:frame frame)
      render-frame! $ :timestamp-ms frame
    (:mouse-down pointer)
      match $ :action
        :target pointer
          (:some action) (dispatch! action)
          (:none) (&unit)
    (:window-close close)
      println $ :reason close
    _ $ println event
```

The native transport first emits a private `PaintEventFfi<Map<Tag, Dynamic>>`
envelope. `paint-event-from-ffi` immediately and strictly decodes that map into
the public structs: missing required fields, wrong nested types, unknown fields,
unknown event variants, and unsupported window operations fail explicitly.
`launch-canvas!` and `launch-canvas-with-options!` remain source-compatible and
continue to deliver their original map protocol.

native transport 会先产生私有的 `PaintEventFfi<Map<Tag, Dynamic>>` envelope，随后
`paint-event-from-ffi` 立即严格解码为公开 struct：缺少必填字段、嵌套类型错误、未知
字段、未知 event variant 或不支持的 window operation 都会明确失败。
`launch-canvas!` 与 `launch-canvas-with-options!` 保持源码兼容，并继续发送原有 map
协议。

### Calcit type boundaries / Calcit 类型边界

Public wrappers use explicit `Unit` returns for side effects. Drawing and
offscreen-export payloads are generic because each operation accepts a
different EDN shape, while text and paragraph measurement return
`Map<Tag, Number>`. Five partially typed definitions remain by design: the two
compatible blocking launch APIs still deliver legacy `nil` and heterogeneous
event maps; text-option and paragraph-option map values are heterogeneous; and
`paint-event-from-ffi` accepts the one raw `Map<Tag, Dynamic>` transport value
before strict nominal decoding. Callback result type `R` remains generic; all
three launch APIs discard that result inside an adapter and
return the serializable `:handled` tag to the blocking ABI because Calcit
`Unit` is intentionally not Cirru EDN. These boundaries are tracked by the
reviewed quality baseline rather than being misrepresented as homogeneous
values or JS FFI.

公开 wrapper 的副作用返回值均显式声明为 `Unit`。不同绘制与离屏导出操作接收不同 EDN
shape，因此 payload 使用泛型；单行文字与段落测量结果均明确为 `Map<Tag, Number>`。
目前仅有五个 partial definition 是有意保留的真实框架边界：两个兼容的 blocking
launch API 仍先送达旧行为的 `nil`、随后送达异构事件 map；单行文字与段落选项 map 的
value 也为异构数据；`paint-event-from-ffi` 则只在严格 nominal 解码前接收一次原始
`Map<Tag, Dynamic>` transport。callback 返回类型 `R` 仍为泛型；三个 launch API
都在内部 adapter 中丢弃该结果，
并向 blocking ABI 返回
可序列化的 `:handled` tag，因为 Calcit `Unit` 本身并不是 Cirru EDN。这些边界由已审核
的质量基线跟踪，而不会被错误标成同构类型或 JS FFI。

### Input events / 输入事件

The compatible launch APIs continue to emit Calcit maps. Their existing
`:type`, `:x`, `:y`, `:dx`, `:dy`, `:action`, `:path`, and `:data` fields are
unchanged. `launch-canvas-typed!` exposes the same semantics through
`PaintEvent` and nominal payload structs. Legacy pointer events now add
`:modifiers`, a map containing `:shift?`, `:control?`, `:alt?`, and `:super?`.
`mouse-down` and `mouse-up` also include `:button`: `:primary`, `:secondary`,
`:middle`, `:back`, `:forward`, or `:other`; `:other` includes numeric
`:button-id`.

兼容 launch API 继续发送 Calcit map，既有的 `:type`、`:x`、`:y`、`:dx`、`:dy`、
`:action`、`:path`、`:data` 字段保持不变；`launch-canvas-typed!` 则通过
`PaintEvent` 与 nominal payload struct 暴露相同语义。旧版指针事件新增
`:modifiers` map，其中包括
`:shift?`、`:control?`、`:alt?`、`:super?`。`mouse-down` 与 `mouse-up` 还会提供
`:button`：`:primary`、`:secondary`、`:middle`、`:back`、`:forward` 或 `:other`；
`:other` 同时提供数字 `:button-id`。

Native desktop file ingress is exposed as `:file-hover`, `:file-drop`, and
`:file-hover-cancel`. Compatible callbacks receive maps; typed callbacks receive
`PaintFileEvent` or `PaintFileHoverCancelEvent`. Hover/drop payloads include a
nominal Calcit `FsPath`, the latest logical `:x` / `:y`, and `PaintModifiers`.
Cancellation deliberately has no fabricated path. Paint never reads, copies, or
uploads a dropped file: the application owns every follow-up filesystem effect.
Host paths that cannot be represented as UTF-8 are rejected with an explicit
stderr diagnostic instead of lossy conversion. The bundled demo is runnable with
the normal launch command—drag a file over the window and drop it to see the
typed lifecycle status update.

桌面原生文件输入以 `:file-hover`、`:file-drop`、`:file-hover-cancel` 暴露。兼容
callback 接收 map，强类型 callback 接收 `PaintFileEvent` 或
`PaintFileHoverCancelEvent`。hover/drop payload 包含 nominal Calcit `FsPath`、最近的
逻辑坐标 `:x` / `:y` 与 `PaintModifiers`；取消事件不会伪造 path。Paint 不会自动读取、
复制或上传被拖入的文件，后续文件系统副作用完全由应用决定。无法表示为 UTF-8 的宿主
路径会通过 stderr 明确拒绝，而不是有损转换。仓库内 demo 可用常规启动命令直接运行；
把文件拖过窗口并放下即可看到强类型生命周期状态变化。

### Text clipboard / 文本剪贴板

`write-clipboard-text!` and `read-clipboard-text!` expose the platform's default
clipboard as a serialized UTF-8 text effect. The write API returns `Unit`; the
read API returns `String`. Initialization, empty/non-text content, occupied
clipboard state, and unsupported desktop environments fail explicitly through
the existing C-safe FFI error path. Image, HTML, file-list, primary-selection,
and background-change polling APIs are deliberately outside this boundary.

`write-clipboard-text!` 与 `read-clipboard-text!` 把平台默认剪贴板暴露为串行 UTF-8
文本副作用；写入返回 `Unit`，读取返回 `String`。初始化失败、空或非文本内容、剪贴板
被占用以及桌面环境不支持等情况，都会通过现有 C-safe FFI 错误路径明确失败。图片、HTML、
文件列表、primary selection 与后台变更轮询不在此边界内。

Paint keeps one lazily initialized clipboard handle so Linux X11/XWayland
selection ownership remains valid while the application runs and Windows access
is serialized. The handle is released when the winit event loop exits. Pure
Wayland data-control is not enabled because compositor support is not universal;
XWayland remains the Linux fallback. The bundled runnable demo uses `Shift+C` to
write bilingual sample text and `Shift+V` to read it back, displaying both states
inside the window.

Paint 保留一个惰性初始化的 clipboard handle，使 Linux X11/XWayland selection 在
应用运行期间保持 ownership，同时串行化 Windows 访问；winit event loop 退出时会释放
该 handle。由于 compositor 支持并不统一，本阶段不启用纯 Wayland data-control，Linux
仍以 XWayland 为兼容路径。仓库内可运行 demo 使用 `Shift+C` 写入双语样例文本，使用
`Shift+V` 读回，并在窗口中显示状态。

`:clicks` is counted separately for each button. A sequence continues when the
next press happens within 500 ms and within four logical pixels; otherwise it
restarts at `1`. `mouse-move` and `mouse-wheel` retain `:clicks` with the most
recent count rather than a hard-coded value.

`:clicks` 会为每个 button 单独计数：下一次按下发生在 500 ms 内且距离不超过四个
逻辑像素时，计数会递增；否则从 `1` 重新开始。`mouse-move` 与 `mouse-wheel`
保留 `:clicks`，其值为最近一次计数，而不再固定为常数。

Paint tracks the topmost, last-drawn `touch-area` under the pointer. Crossing
between targets emits `:pointer-leave` before `:pointer-enter`; both events carry
`:x`, `:y`, `:modifiers`, `:action`, `:path`, `:data`, `:cursor`, and
`:captured? false`. Stable scene paths preserve hover across redraws. If the
hovered target is removed, Paint emits its leave event and then enters the newly
revealed target at the same position. Leaving the window emits the compatible
`:mouse-leave` plus the target's `:pointer-leave`, and restores the default
system cursor.

Paint 跟踪指针下最后绘制、位于最上层的 `touch-area`。跨目标时先发送
`:pointer-leave`，再发送 `:pointer-enter`；两者均包含 `:x`、`:y`、`:modifiers`、
`:action`、`:path`、`:data`、`:cursor` 与 `:captured? false`。稳定 scene path 会让
hover 在重绘之间保持不变。若 hovered target 被移除，Paint 会先发送其 leave，再进入
同一位置新露出的目标。指针离开窗口时仍发送兼容的 `:mouse-leave`，同时发送目标的
`:pointer-leave`，并恢复默认系统光标。

Both topmost selection and hover reconciliation honor every ancestor
`clip-rect` / `clip-rounded-rect`. Changing a clip while the pointer is
stationary can therefore emit leave/enter on the next rendered scene. An
already established pointer capture continues routing to its target while that
target remains mounted; releasing capture immediately reconciles against the
new clip stack.

最上层目标选择与 hover reconcile 都会遵守所有祖先 `clip-rect` /
`clip-rounded-rect`。因此，即使指针静止，下一次 scene 渲染改变 clip 后也可能发送
leave/enter。已经建立的 pointer capture 在目标仍存在时继续路由；释放 capture 后会立刻
按新的 clip stack 重新计算 hover。

Pressing a mouse button inside a touch area establishes pointer capture for that
button. Captured `:mouse-move` and the matching `:mouse-up` continue to use the
original target outside its hit geometry and include `:captured? true`, the
button, and drag `:dx`/`:dy`. Release ends capture and immediately reconciles
hover. Target removal, window exit, or window-focus loss emits
`:pointer-cancel` with `:cancelled? true` and reason `:target-removed`,
`:window-leave`, or `:window-blur`. Capture is event-routing semantics; it does
not confine or lock the physical cursor.

在 touch area 内按下鼠标会为该 button 建立 pointer capture。即使移出命中几何，
被捕获的 `:mouse-move` 与对应 `:mouse-up` 仍路由给原目标，并包含
`:captured? true`、button 以及拖拽 `:dx`/`:dy`。释放后结束 capture 并立即重新计算
hover。目标移除、指针离开窗口或窗口失焦时会发送 `:pointer-cancel`，其中包含
`:cancelled? true`，reason 分别为 `:target-removed`、`:window-leave` 或
`:window-blur`。Capture 仅定义事件路由，不会限制或锁定物理光标。

The bundled runnable demo contains overlapping `:grab` and `:crosshair` areas,
plus a translated touch target partially hidden by a rounded clip. Move across
the visible clip edge, then test the invisible continuation outside it; hover
and the system cursor stop exactly at the painted boundary. Press and drag
outside a target to verify that capture still routes until release. The status
line shows all transitions, with updates coalesced through `request-frame!`.

随仓库提供的可运行 demo 包含重叠的 `:grab` / `:crosshair` 区域，以及部分隐藏在圆角
clip 内、经过 translate 的 touch target。沿可见裁剪边缘移动，再测试边缘外不可见的延伸
区域，hover 与系统光标会准确止于绘制边界；按下后拖出目标则仍会捕获到释放为止。状态行
展示全部转换，并通过合并的 `request-frame!` 更新。

Keyboard events include layout-aware `:name`, legacy numeric `:key-code`,
portable `:physical-key`, and `:modifiers`. `:name` is the logical key and can
vary with the active keyboard layout; single-character names retain the
uppercase behavior used by existing listeners. `:physical-key` is a winit
physical code name such as `"KeyD"`; for unknown hardware it is an
`"Unidentified(...)"` string containing a platform-native code. Keep
cross-platform shortcuts on `:name`; use `:physical-key` only when a physical
layout-independent binding is required. The legacy number is retained for
compatibility but should not be used as a portable identifier.

键盘事件包含布局相关的 `:name`、旧版数字 `:key-code`、可移植的 `:physical-key`
和 `:modifiers`。`:name` 是逻辑键名，会受当前键盘布局影响；单字符键名仍保持旧
listener 使用的大写行为。`:physical-key` 是 winit 的物理键名，例如 `"KeyD"`；
未知硬件会使用包含平台原生 code 的 `"Unidentified(...)"` 字符串。跨平台快捷键
优先使用 `:name`；只有需要与物理键位无关的绑定时才使用 `:physical-key`。旧版数字
字段为兼容保留，不应作为可移植标识。

#### Focus, shortcuts, and IME / 焦点、快捷键与 IME

A `:focus-area` uses the same circle (`:radius`) or centered rectangle (`:dx`
and `:dy`) geometry as `:touch-area`. It requires a stable string
`:focus-id`; optional `:tab-index` defaults to `0`, and negative values exclude
the area from Tab traversal. Equal indices retain render registration order.
`:text-input? true` enables the platform IME only while that area owns focus.
Duplicate focus IDs in one rendered scene are rejected.

`:focus-area` 使用与 `:touch-area` 相同的圆形（`:radius`）或中心矩形（`:dx`、
`:dy`）几何。它要求稳定的字符串 `:focus-id`；可选 `:tab-index` 默认为 `0`，负值
表示不参加 Tab 导航，相同 index 按渲染注册顺序排列。只有获得焦点且声明
`:text-input? true` 的区域会启用平台 IME。同一帧出现重复 focus ID 会直接报错。

```cirru.no-check
{} (:type :focus-area) (:focus-id |field-a) (:tab-index 0) (:text-input? true)
  :position $ [] 180 450
  :dx 140
  :dy 32
  :action :focus-demo
  :fill-color $ [] 215 70 45

{} (:type :key-listener) (:key |K) (:action :focus-first)
  :modifiers $ {} (:shift? true)

{} (:type :key-listener) (:key |Enter) (:focus-id |field-a) (:action :field-submit)
```

Primary-clicking an area transfers focus; clicking outside all focus areas
clears it. Tab and Shift+Tab traverse and wrap. Focus transitions emit
`:focus-in`/`:focus-out` with `:focus-id`, `:related-focus-id`, `:reason`, and
the target's existing `:action`/`:path`/`:data`. Reasons are `:pointer`,
`:tab`, `:programmatic`, `:escape`, `:window-blur`, or `:removed`. Removing a
focused node on the next rendered frame clears focus rather than leaving a
stale owner. Window blur clears focus; Escape first cancels composition and
clears focus, and retains the historical exit behavior only when no focus or
composition is active.

主键点击区域会转移焦点，点击所有 focus area 之外会清除焦点；Tab 与 Shift+Tab
循环导航。焦点变化发送 `:focus-in`/`:focus-out`，包含 `:focus-id`、
`:related-focus-id`、`:reason` 及目标原有的 `:action`/`:path`/`:data`。reason
可能为 `:pointer`、`:tab`、`:programmatic`、`:escape`、`:window-blur` 或
`:removed`。下一帧移除已聚焦节点会清除焦点，不会留下失效 owner；窗口失焦也会
清除焦点。Escape 会先取消 composition 并清焦点，仅在没有焦点和 composition 时保留
历史上的退出窗口行为。

Calcit can request and release focus with `focus!` and `blur!`, and query one
ID with `focused?`. `focus!` rejects an ID that is not registered in the
current rendered scene. Programmatic transition events are delivered after the
current callback returns, avoiding reentrant Calcit callbacks.

Calcit 可通过 `focus!`、`blur!` 请求或释放焦点，并用 `focused?` 查询指定 ID。
`focus!` 会拒绝当前已渲染 scene 中不存在的 ID。程序化焦点事件会在当前 callback
返回后投递，避免 Calcit callback 重入。

```cirru.no-check
focus! |field-a

focused? |field-a

blur!
```

An old `:key-listener` without `:modifiers` remains a wildcard over modifier
state. Supplying a `:modifiers` map makes all four flags (`:shift?`,
`:control?`, `:alt?`, `:super?`) an exact chord; omitted flags mean `false`.
Matched chord events add `:shortcut? true`. Optional `:focus-id` restricts a
listener to that current owner. This preserves legacy single-key listeners
while making shortcuts deterministic.

旧 `:key-listener` 不提供 `:modifiers` 时仍忽略 modifier 状态，保持兼容。显式提供
`:modifiers` map 后，`:shift?`、`:control?`、`:alt?`、`:super?` 四个 flag 会做
精确 chord 匹配，省略的 flag 表示 `false`；命中事件增加 `:shortcut? true`。可选
`:focus-id` 会把 listener 限定到当前焦点 owner。

For a focused text-input area, winit IME events map to `:ime-enabled`,
`:composition-start`, `:composition-update`, `:composition-end`,
`:text-input`, and `:ime-disabled`. Composition events carry `:text`,
`:cursor-start`, and `:cursor-end`; cursor offsets follow winit and are UTF-8
byte indices, not Unicode scalar indices. `:composition-end` includes
`:cancelled?`; committed insertion is the `:text-input` event. Focus transfer,
Escape, window blur, IME disable, and node removal explicitly cancel any live
composition. Platform IMEs differ in whether they emit Enabled/Disabled and in
their exact preedit sequence, so consumers must rely on the lifecycle contract
rather than a fixed event count.

当文本输入区域获得焦点后，winit IME 会映射为 `:ime-enabled`、
`:composition-start`、`:composition-update`、`:composition-end`、`:text-input`
与 `:ime-disabled`。composition 事件包含 `:text`、`:cursor-start`、
`:cursor-end`；cursor offset 遵循 winit，是 UTF-8 字节索引而非 Unicode scalar
索引。`:composition-end` 带 `:cancelled?`，最终提交文本由 `:text-input` 表示。
焦点转移、Escape、窗口失焦、IME disable 与节点卸载都会显式取消未完成 composition。
不同平台是否发送 Enabled/Disabled 及具体 preedit 序列可能不同，消费端应依赖生命周期
契约而非固定事件数量。

Run the bundled scene with `calcit ./calcit.cirru`, then click or drag the
"Pointer event demo" panel, hold a modifier, or press `I`. Two additional
focus areas demonstrate click focus, Tab/Shift+Tab traversal, IME input,
focus-scoped Enter, and a Shift+K shortcut that calls `focus!`. The callback
prints live nominal events to the Calcit terminal. Shift+P explicitly exports
the offscreen demo PNG. Shift+C writes the clipboard sample and Shift+V reads it
back into the visible demo status.

运行 `calcit ./calcit.cirru` 后，点击或拖拽 “Pointer event demo” 面板、按住任意
modifier，或按下 `I`。新增的两个 focus area 还会实际演示点击聚焦、Tab/Shift+Tab、
IME 输入、焦点限定 Enter，以及调用 `focus!` 的 Shift+K 快捷键；callback 会把实时
nominal event 打印到 Calcit terminal。Shift+P 会显式导出离屏 demo PNG。
Shift+C 会写入剪贴板样例，Shift+V 会将其读回到可见 demo 状态。

- Rotate

`radius` is an angle in radians. Paint converts it to Skia's degree-based
canvas API while retaining radians in hit-test transforms, so drawing and
interaction share the same rotation.

`radius` 是以弧度表示的角度。Paint 会为 Skia 以 degree 为单位的 canvas API 做转换，
同时在命中 transform 中保留 radians，因此绘制与交互使用同一旋转。

```rust
Rotate {
  radius: f32,
}
```

- Translate

```rust
Translate {
  x: f32,
  y: f32,
}
```

- Scale

```rust
Scale {
  factor: f32,
}
```

### License

MIT
