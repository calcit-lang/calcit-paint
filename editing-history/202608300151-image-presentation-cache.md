# Image presentation and bounded decoded cache / 图片展示与有界解码缓存

## 中文

- `image` 新增严格 tag 字段 `:fit`（`:fill`、`:contain`、`:cover`）与 `:sampling`（`:nearest`、`:linear`、`:cubic`）。省略或传 `nil` 时分别保持旧版 `:fill` 与 `:nearest` 行为。
- `contain` 在目标矩形中居中并完整显示源图；`cover` 居中裁切源区域后填满目标矩形。可选 `:crop` 先限定源区域，再参与 fit 计算。
- 目标位置必须有限、目标尺寸必须为有限正数；crop 必须是 map 或 `nil`，坐标为有限非负数且尺寸为有限正数。真实图片边界在解码后校验，错误包含结构化 scene path。
- Skia 绘制改用显式 sampling options；`:cubic` 采用 Mitchell resampler，带源区域的绘制使用 strict source constraint，避免越界采样。
- 解码图片缓存升级为 LRU：最多 64 项、64 MiB 预估 RGBA 内存。缓存仍按文件大小和 mtime 失效；超限单图仍渲染但不缓存，文件消失时清除旧项并保持原有 log-and-skip 行为。
- 默认 `calcit.cirru` 增加可运行三联 demo，实际覆盖兼容 fill/nearest、contain/linear，以及带 crop 的 cover/cubic。
- 补充解析默认值、fit 几何、sampling 映射、真实 PNG 离屏像素、crop 越界路径诊断、LRU entry/byte 上限与异常输入测试。

## English

- Added strict-tag `:fit` (`:fill`, `:contain`, `:cover`) and `:sampling` (`:nearest`, `:linear`, `:cubic`) fields to `image`. Missing or `nil` values preserve the legacy `:fill` and `:nearest` behavior.
- `contain` centers the complete source inside the destination; `cover` center-crops the source before filling the destination. An optional `:crop` first restricts the source region used by fit calculations.
- Destination positions must be finite and destination dimensions finite positive values. Crop must be a map or `nil`, with finite non-negative coordinates and finite positive dimensions. Actual decoded bounds are checked after loading, with the structural scene path in failures.
- Image drawing now passes explicit Skia sampling options. `:cubic` uses the Mitchell resampler, and source-rect draws use the strict source constraint to prevent out-of-bounds sampling.
- The decoded-image cache is now an LRU capped at 64 entries and 64 MiB of estimated RGBA memory. File size and mtime still invalidate entries; oversized images render without being cached, while missing files evict stale entries and retain the legacy log-and-skip behavior.
- The default `calcit.cirru` contains a runnable three-panel demo exercising compatible fill/nearest, contain/linear, and cropped cover/cubic.
- Added coverage for parser defaults, fit geometry, sampling mapping, real-PNG offscreen pixels, crop-bound path diagnostics, entry/byte LRU limits, and invalid inputs.

## Verification / 验证

- `cargo fmt --check`
- `cargo test` (64 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README check-md (19 blocks), quality baseline, and dynamic-method gate
- Release `./build.sh`
- Real auto-exit macOS Calcit window smoke rendering the maintained demo
- The existing Linux/Xvfb Actions blocking-canvas smoke runs the same maintained demo

Local Rust builds used the verified `file:///private/tmp/skia-binaries-{key}.tar.gz`
archive because the configured upstream macOS archive URL returned 404 during this
session. / 本地 Rust 构建使用已验证的上述 Skia 二进制归档；本次会话中配置的
上游 macOS 归档 URL 返回 404。
