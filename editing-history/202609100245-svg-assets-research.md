# SVG asset research harness / SVG 资源调研 harness

## English

- Conclude the 0.1.0 SVG research without exposing a public Calcit scene node:
  Skia SVG renders successfully, but opaque parse failures, underspecified fit
  semantics, DOM caching, resource limits, and missing binary-cache coverage
  are not yet acceptable public contracts.
- Add a non-default `svg-research` Cargo feature, local fixture, raster assertions,
  and an environment-gated native overlay so the evidence remains reproducible
  without changing the default dylib or Calcit API.
- Add a path-scoped Linux workflow using exact `setup-calcit v1.5.0` to record
  baseline/SVG release sizes and run both raster and Xvfb native probes.
- Record arm64 macOS capability, failure behavior, sizing semantics, artifact
  sizes, build observations, alternatives, and explicit gates for reconsidering
  a strict local-only SVG resource API.

## 中文

- 以“不公开 Calcit scene node”收口 0.1.0 SVG 调研：Skia SVG 可成功绘制，但 opaque
  parse failure、未明确的 fit 语义、DOM cache、资源限制和缺失 binary-cache 覆盖尚不能
  成为可靠公开契约。
- 增加默认关闭的 `svg-research` Cargo feature、本地 fixture、raster 断言和环境变量控制
  的 native overlay，使证据可以复验，同时不改变默认 dylib 或 Calcit API。
- 增加 path-scoped Linux workflow，使用精确 `setup-calcit v1.5.0` 记录 baseline/SVG
  release 体积，并执行 raster 与 Xvfb native probes。
- 记录 arm64 macOS 能力、失败行为、尺寸语义、artifact 体积、构建观察、替代方案，以及
  重新考虑严格本地 SVG 资源 API 的明确门槛。
