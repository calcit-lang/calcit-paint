## (Toy)Calcit Paint

> 2D renderer for Calcit.

### Usages / 使用方式

It runs [Calcit](https://github.com/calcit-lang/calcit) and is driven by the canonical `calcit.cirru` Snapshot source.

项目直接运行 [Calcit](https://github.com/calcit-lang/calcit)，并以规范的
`calcit.cirru` Snapshot 作为源码。默认场景包含可直接看到效果的渐变、虚线描边和
混合模式 demo。

```bash
./build.sh
calcit ./calcit.cirru
```

Available APIs:

```cirru.no-check
calcit-paint.core/push-drawing-data! |reset-canvas! nil
calcit-paint.core/push-drawing-data! |render-canvas! shape-data

calcit-paint.core/launch-canvas! $ fn (event)
  println "|rendering to canvas..."
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

- Text, using `text`

```rust
Text {
  text: String,
  position: Vec2,
  size: f32,
  // weight: String, // TODO
  color: Color,
  // align: TextAlign,
},
```

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

Window callbacks also emit `:mouse-wheel` events with `:dx`, `:dy`, and a
`:unit` of either `:line` or `:pixel`. Pixel deltas are normalized to logical
pixels, consistently with drawing coordinates.

For handling events:

```rust
TouchArea {
  path: Calcit,
  action: Calcit,
  data: Calcit,
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
  key: String, // TODO modifier
  action: Calcit,
  path: Calcit,
  data: Calcit,
  // children: Vec<Shape>, // TODO
},
```

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
