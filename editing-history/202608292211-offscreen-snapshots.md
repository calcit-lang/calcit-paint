# Offscreen rendering and deterministic snapshots / 离屏渲染与确定性快照

## 中文

- 抽取同一套 shape 递归的 interactive/offscreen 模式，新增显式 RGBA8888 premultiplied
  sRGB CPU raster surface；离屏路径不污染 touch、key 或 focus registry。
- 新增 `render-to-png!` Calcit wrapper 与 `render_to_png` C-safe buffer ABI。只有调用方提供
  明确路径时才写文件，并限制为 1–4096 的整数宽高及最多 16,777,216 像素。
- 新增 `:cached-group` / `:static-group` 静态子树原型：按 key、revision 与尺寸命中，使用
  32-entry / 32 MiB process-wide LRU；交互子节点会报错，children 或资源变化需显式递增 revision。
- 默认 Calcit scene 显示真实 cached badge，并提供 Shift+P 显式导出 `offscreen-demo.png`；
  默认启动不产生文件，demo 输出加入 `.gitignore`。
- 确定性测试锁定透明背景、关键 RGBA 像素与完整 FNV-1a pixel hash，并覆盖 PNG header/
  尺寸、非法输入、revision 失效、交互拒绝和 LRU 淘汰。
- CI 新增 Linux Calcit→FFI→PNG smoke，并继续保留窗口 Xvfb smoke 与 C-safe export audit。

## English

- Split the shared shape recursion into interactive and offscreen modes and added an explicit
  RGBA8888 premultiplied-sRGB CPU raster surface. Offscreen rendering does not mutate touch, key,
  or focus registries.
- Added the `render-to-png!` Calcit wrapper and `render_to_png` C-safe buffer ABI. Files are written
  only to a caller-provided path, with integer dimensions from 1–4096 and a 16,777,216-pixel cap.
- Added the `:cached-group` / `:static-group` static-subtree prototype keyed by key, revision, and
  dimensions, backed by a process-wide 32-entry / 32 MiB LRU. Interactive descendants fail
  explicitly, and callers bump revision when children or resources change.
- Extended the default Calcit scene with a real cached badge and Shift+P explicit export to
  `offscreen-demo.png`. Startup itself writes no file, and the demo output is gitignored.
- Added deterministic assertions for transparent background, key RGBA pixels, and a complete
  FNV-1a pixel hash, plus PNG header/dimensions, invalid inputs, revision invalidation, interaction
  rejection, and LRU eviction coverage.
- Added a Linux Calcit-to-FFI-to-PNG CI smoke while retaining the window Xvfb smoke and C-safe
  export audit.

## Verification / 验证

- `cargo fmt --all -- --check`
- `cargo test` (42 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README check-md, quality baseline, and dynamic-method gate
- `./build.sh`, macOS C-safe export audit, explicit real Calcit PNG export, and default window smoke
