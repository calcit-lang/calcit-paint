## (Toy)Calcit Paint

> 2D renderer for Calcit.

### Usages / 使用方式

It runs [Calcit](https://github.com/calcit-lang/calcit) and is driven by the canonical `calcit.cirru` Snapshot source.

项目直接运行 [Calcit](https://github.com/calcit-lang/calcit)，并以规范的
`calcit.cirru` Snapshot 作为源码。默认场景包含可直接看到效果的渐变、虚线描边和
混合模式、输入事件和文本排版 demo。

```bash
./build.sh
calcit ./calcit.cirru
```

Available APIs:

```cirru.no-check
calcit-paint.core/push-drawing-data! |reset-canvas! nil
calcit-paint.core/push-drawing-data! |render-canvas! shape-data
calcit-paint.core/measure-text! text-options

calcit-paint.core/launch-canvas! $ fn (event)
  println "|rendering to canvas..."
  &unit
```

### Native FFI / 原生 FFI

The C-safe buffer-v1 and blocking-callback descriptors, ownership rules, Cirru
EDN transport, and adapters are provided by
[`calcit_native_ffi`](https://github.com/calcit-lang/calcit-native-ffi). Paint
keeps ownership of the Skia/winit event loop, rendering state, shape decoding,
and callback scheduling. The module tracks `calcit_native_ffi 0.1.2`; buffer
and blocking-callback protocols remain at v1. It requires Calcit 0.13.58.

C-safe buffer-v1/blocking-callback descriptor、ownership 规则、Cirru EDN
transport 与 adapter 由
[`calcit_native_ffi`](https://github.com/calcit-lang/calcit-native-ffi) 统一维护。
Paint 仍负责 Skia/winit 事件循环、绘制状态、shape 解析和回调调度；
模块当前使用 `calcit_native_ffi 0.1.2`，buffer 与 blocking-callback protocol
均继续保持 v1；模块要求 Calcit 0.13.58。

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
{} (:type :rounded-rect) (:position ([] 20 20))
  :width 120
  :height 60
  :radius 16
  :fill-color $ [] 210 70 55
```

- Ellipse, using `ellipse`, with `radius-x` and `radius-y`.

- Arc, using `arc`. Angles are in degrees; positive sweep is clockwise.
  `use-center? true` draws a wedge:

```cirru
{} (:type :arc) (:position ([] 200 120))
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
measure-text! $ {}
  :text "|Text layout / 文本排版"
  :size 24
  :font-family |monospace
  :weight 700
  :style :italic
  :baseline :middle
```

Run `./build.sh` followed by `calcit ./calcit.cirru` to run the maintained
Calcit demo. It prints the measurement map before opening the canvas, then
displays bold italic/top, regular/middle, and light/bottom text samples.

执行 `./build.sh` 后运行 `calcit ./calcit.cirru` 即可启动维护中的 Calcit demo。
它会在打开画布前打印测量 map，并显示粗斜体/top、常规/middle、细体/bottom 三组文本。

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
  [] :move-to ([] 1 2)
  [] :line-to ([] 3 4)
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

- Image, using `image`, and `Rect` from Skia

```rust
Image {
  file_path: String,
  x: f32, y: f32, w: f32, h: f32,
  crop: Rect {
    x: f32, y: f32, w: f32, h: f32
  }
}
```

Decoded images are cached and automatically reloaded when file size or
modification time changes.

- Clip rect, using `clip-rect`, clips all children to its rectangular bounds.

- Opacity, using `opacity`, composites all children as one layer. `alpha` must
  be between `0` and `1`:

```cirru
{} (:type :opacity) (:alpha 0.6)
  :children $ []
    {} (:type :circle) (:position ([] 80 80)) (:radius 40)
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
  line_style: Option<StrokeStyle>,
  fill_style: Option<PaintSource>,
},
```

- Key listener, using `key-listener`

```rust
KeyListener {
  key: String,
  target: EventTarget {
    action: Option<Calcit>,
    path: Option<Calcit>,
    data: Option<Calcit>,
  },
  // children: Vec<Shape>, // TODO
},
```

`:action`, `:path`, and `:data` are optional on both shape maps. Omitting a
field and passing an explicit `nil` are equivalent. Internally Paint stores
them as `Option<Edn>`; when a touch area or key listener matches, the emitted
event still contains all three legacy keys and uses `nil` for absent values.
This keeps existing event consumers compatible while allowing new Calcit demos
to omit meaningless placeholders.

`:action`、`:path` 与 `:data` 在两类 shape map 中均为可选字段；省略字段与显式传入
`nil` 等价。Paint 内部使用 `Option<Edn>` 存储；当 touch area 或 key listener
命中时，发出的事件仍保留这三个历史字段，缺失值以 `nil` 表示。因此旧事件消费代码
保持兼容，而新的 Calcit demo 不再需要填写无意义的占位值。

### Calcit type boundaries / Calcit 类型边界

Public wrappers use explicit `Unit` returns for side effects. Drawing payloads
are generic because each operation accepts a different EDN shape, while text
measurement returns `Map<Tag, Number>`. Two `Dynamic` slots remain by design:
the blocking callback first receives legacy `nil` and then heterogeneous event
maps, while text-option map values are heterogeneous. Callback result type `R`
remains generic; `launch-canvas!` discards that result inside an adapter and
returns the serializable `:handled` tag to the blocking ABI because Calcit
`Unit` is intentionally not Cirru EDN. These boundaries are tracked by the
reviewed quality baseline rather than being misrepresented as homogeneous
values or JS FFI.

公开 wrapper 的副作用返回值均显式声明为 `Unit`。不同绘制操作接收不同 EDN shape，
因此 drawing payload 使用泛型；文字测量结果则明确为 `Map<Tag, Number>`。目前仅有
两个 `Dynamic` 是有意保留的真实框架边界：blocking callback 会先收到兼容旧行为的
`nil`、随后收到异构事件 map；文字选项 map 的 value 也为异构数据。callback 返回类型
`R` 仍为泛型；`launch-canvas!` 在内部 adapter 中丢弃该结果，并向 blocking ABI 返回
可序列化的 `:handled` tag，因为 Calcit `Unit` 本身并不是 Cirru EDN。这些边界由已审核
的质量基线跟踪，而不会被错误标成同构类型或 JS FFI。

### Input events / 输入事件

Every event remains a Calcit map. Existing `:type`, `:x`, `:y`, `:dx`, `:dy`,
`:action`, `:path`, and `:data` fields are unchanged. Pointer events now add
`:modifiers`, a map containing `:shift?`, `:control?`, `:alt?`, and `:super?`.
`mouse-down` and `mouse-up` also include `:button`: `:primary`, `:secondary`,
`:middle`, `:back`, `:forward`, or `:other`; `:other` includes numeric
`:button-id`.

所有事件仍是 Calcit map。既有的 `:type`、`:x`、`:y`、`:dx`、`:dy`、`:action`、
`:path`、`:data` 字段保持不变。指针事件新增 `:modifiers` map，其中包括
`:shift?`、`:control?`、`:alt?`、`:super?`。`mouse-down` 与 `mouse-up` 还会提供
`:button`：`:primary`、`:secondary`、`:middle`、`:back`、`:forward` 或 `:other`；
`:other` 同时提供数字 `:button-id`。

`:clicks` is counted separately for each button. A sequence continues when the
next press happens within 500 ms and within four logical pixels; otherwise it
restarts at `1`. `mouse-move` and `mouse-wheel` retain `:clicks` with the most
recent count rather than a hard-coded value.

`:clicks` 会为每个 button 单独计数：下一次按下发生在 500 ms 内且距离不超过四个
逻辑像素时，计数会递增；否则从 `1` 重新开始。`mouse-move` 与 `mouse-wheel`
保留 `:clicks`，其值为最近一次计数，而不再固定为常数。

An active touch-area drag is attached only to the button that started it. When
the cursor leaves the window, Paint emits `:mouse-leave`; if a drag is active,
the event includes its `:action`, `:path`, `:data`, `:button`, `:dx`, `:dy`, and
`:cancelled? true`, then clears the drag. Consumers should use this event to
finish drag state when a physical mouse-up occurs outside the window.

touch-area drag 只会关联启动它的那个 button。光标离开窗口时，Paint 会发送
`:mouse-leave`；若 drag 正在进行，事件会包含其 `:action`、`:path`、`:data`、
`:button`、`:dx`、`:dy` 和 `:cancelled? true`，随后清理 drag。消费者应利用该
事件完成窗口外松开鼠标时的清理。

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

Run the bundled scene with `calcit ./calcit.cirru`, then click or drag the
"Pointer event demo" panel, hold a modifier, or press `I`. The callback prints
the live event map, including the new fields, to the Calcit terminal.

运行 `calcit ./calcit.cirru` 后，点击或拖拽 “Pointer event demo” 面板、按住任意
modifier，或按下 `I`。回调会在 Calcit terminal 中打印包含新字段的实时事件 map。

- Rotate

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
