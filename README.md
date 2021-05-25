## Calcit Paint

> 2D renderer for calcit_runner.

Previous work: https://github.com/calcit-lang/edn-paint .

### Usages

_TODO_ not stable yet...

```bash
cargo install calcit_paint
```

It imports [calcit_runner](https://github.com/calcit-lang/calcit_runner.rs) inside and is supposed to be driven by `compact.cirru` code.

```bash
cargo run calcit/compact.cirru

# or adding entry
cargo run -- calcit/compact.cirru --event-entry "app.main/main!"
```

### Shapes

Position represented with `[] x y`. Color with `[] h s l a?`

Drawing with lyon `0.17.5`:

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

_Arc is not supported at current._

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
  path: Vec<PaintPath>,
  line_style: Option<(Color, f32)>,
  fill_style: Option<Color>,
},
```

It's rendered based on lyon SVG path builder. Only small subset:

```cirru
[]
  [] :move-to ([] 1 2)
  [] :line-to ([] 3 4)
  [] :quadratic-bezier-to ([] 5 6) ([] 7 8)
  [] :cubic-bezier-to ([] 1 2) ([] 3 4) ([] 5 6)
```

Since then name is too long, I also use alies:
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

- Touch Area, using `touch-area`

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

### License

MIT
