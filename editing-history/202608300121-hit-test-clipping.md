# Hierarchical clipping and hit-test consistency / 层级裁剪与命中一致性

## 中文

- 新增共享的 `ClipRegion` / `ClipShape` 几何层；每个 clip 保存其局部形状与当时的完整 transform，嵌套 clip 以交集方式约束命中。
- `clip-rect` 现在同时约束 Skia 绘制、touch hover/cursor/down 与 pointer focus，不再允许裁剪区外不可见内容被点击或聚焦。
- 新增 `clip-rounded-rect`（兼容别名 `clip-rounded-rectangle`），支持严格校验的 `:radius` 或 `:radius-x` / `:radius-y`；视觉裁剪和交互裁剪使用一致的 Skia 半径收窄规则。
- 宽或高为零、不可逆 transform 等边界会安全地产生空命中；嵌套与变换后的 clip 均有单元测试。
- 保持 pointer capture 契约：capture 建立后，只要目标仍存在，即使 clip 改变也继续路由；release 后立即按最新 clip stack reconcile hover。
- focus 的 pointer hit 遵守 clip，但键盘 Tab 与程序化 focus 的逻辑顺序不受视觉裁剪影响。
- 修正既有 rotate 语义：Calcit `:radius` 与 Euclid hit transform 以 radians 表示，传入以 degrees 为单位的 Skia Canvas 前显式转换，消除旋转绘制与命中错位。
- 默认 `calcit.cirru` 将旧矩形 clip 升级为可运行的圆角 clip demo，并加入部分越过边界、经过 translate 的 `:pointer` touch target，沿可见边缘可直接验证 cursor/hover 截止位置。
- 双语 README 记录严格 schema、层级/transform 语义、capture 边界、focus 边界与可运行 demo。

## English

- Added shared `ClipRegion` / `ClipShape` geometry. Each clip retains its local shape and full transform at registration time; nested clips constrain hits by intersection.
- `clip-rect` now constrains Skia painting, touch hover/cursor/down, and pointer-triggered focus, preventing invisible clipped descendants from remaining interactive.
- Added `clip-rounded-rect` with the compatible `clip-rounded-rectangle` alias and strict `:radius` or `:radius-x` / `:radius-y` validation. Visual and interactive clipping share Skia-compatible radius clamping.
- Zero-area clips, singular transforms, nesting, and transformed clips fail closed and have explicit unit coverage.
- Preserved pointer-capture behavior: after capture begins, routing continues while the target remains mounted even if its clip changes; release immediately reconciles hover against the latest clip stack.
- Pointer focus honors clips, while keyboard Tab order and programmatic focus remain logical rather than visibility-based.
- Fixed the existing rotate-unit mismatch: Calcit `:radius` and Euclid hit transforms use radians, which are now converted before calling Skia Canvas's degree-based rotation API.
- Upgraded the default `calcit.cirru` scene to a runnable rounded-clip demo with a translated `:pointer` touch target extending beyond the visible edge, making cursor/hover clipping directly testable.
- Documented strict schemas, hierarchy/transforms, capture and focus boundaries, and the runnable demo in the bilingual README.

## Verification / 验证

- `cargo fmt --all -- --check`
- `cargo test` (61 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README check-md (19 blocks), quality baseline, and dynamic-method gate
- Release `./build.sh`
- Real auto-exit macOS Calcit window smoke with FFI tracing and zero leaked callback buffers
- The same maintained runnable scene remains covered by the existing Linux/Xvfb Actions blocking-canvas smoke
