## (Toy)Calcit Paint

> 2D renderer for Calcit.

### Usages

It runs [Calcit](https://github.com/calcit-lang/calcit) and is driven by the canonical `calcit.cirru` Snapshot source.

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

Position represented with `[] x y`. Color with `[] h s l a?`

Drawing is backed by Skia and uses logical pixels. Shape maps support the
following primitives and containers:

- Rect, using `rect` or `rectangle`:

```rust
rect {
  position: Vec2,
  width: f32,
  height: f32,
  line_style: Option<(Color, f32)>,
  fill_style: Option<Color>,
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
  line_style: Option<(Color, f32)>,
  fill_style: Option<Color>,
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
  line_style: Option<(Color, f32)>,
  fill_style: Option<Color>,
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
  color: Color,
  width: f32,
  join: LineJoin,
  cap: LineCap,
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
  line_style: Option<(Color, f32)>,
  fill_style: Option<Color>,
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
