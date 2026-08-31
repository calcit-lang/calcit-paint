# Executable scene cookbook / 可执行场景手册

## Summary / 概要

- Added a bilingual, stable cookbook index for the smallest useful Calcit Paint
  scenes: shapes, transform/clip, touch/focus composition, typed events,
  accessibility, offscreen PNGs, and local assets.
- 新增双语、稳定的 cookbook 索引，覆盖最小可用的 Calcit Paint 场景：图元、
  transform/clip、touch/focus 组合、强类型事件、无障碍、离屏 PNG 和本地资源。

## Developer and Agent workflow / 开发者与 Agent 工作流

- Keep the large default demo as integration coverage, but start authoring from
  a minimal capability recipe. Discover the public validation recipe through
  `calcit query examples calcit-paint.core/validate-scene`.
- 保留大型默认 demo 作为集成覆盖，但编写时从最小能力 recipe 开始。通过
  `calcit query examples calcit-paint.core/validate-scene` 发现公开校验示例。
- Native FFI snippets in Markdown are intentionally marked `cirru.no-check`:
  documentation checks run before the native dylib is built and must be
  side-effect free. `scripts/check-cookbook.sh` is the executable source of
  truth after the build, validating public API calls and removing its PNG.
- Markdown 中的 native FFI 片段刻意标记为 `cirru.no-check`：文档检查发生在 native
  dylib 构建之前，且必须无副作用。构建后的 `scripts/check-cookbook.sh` 是可执行的
  source of truth，它通过公开 API 校验并清理生成的 PNG。

## Verification / 验证

- CI now checks cookbook formatting, documentation, the API-attached example,
  and the executable public-API cookbook smoke after building the dylib.
- CI 现在检查 cookbook 格式、文档、附着于 API 的 example，并在构建 dylib 后运行
  可执行的公开 API cookbook smoke。
- The cookbook smoke was run with the pinned Calcit 0.13.64; full Rust tests,
  strict Clippy, release build, Calcit quality gates, and native smoke passed.
- cookbook smoke 已使用固定的 Calcit 0.13.64 执行；全量 Rust tests、strict Clippy、
  release build、Calcit quality gates 与 native smoke 均通过。
