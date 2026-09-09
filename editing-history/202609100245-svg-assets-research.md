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
- Resolve review feedback by replacing the two movable workflow action refs
  with exact published tags (`checkout v7.0.1` and `setup-rust-toolchain
  v2.0.0`) and documenting the touched rendering helpers. Per repository
  policy, release tags remain the version source of truth rather than bare
  commit hashes.
- Record arm64 macOS capability, failure behavior, sizing semantics, artifact
  sizes, build observations, alternatives, and explicit gates for reconsidering
  a strict local-only SVG resource API.
- Linux Actions run 34389883340 passed the release raster test and Xvfb native
  overlay. It measured 33,603,416 baseline bytes versus 34,304,096 SVG bytes
  (+700,680 / +2.09%) and 139.55 s versus 1,004.46 s observed build time,
  demonstrating the missing fast binary path for this SVG feature combination.

## 中文

- 以“不公开 Calcit scene node”收口 0.1.0 SVG 调研：Skia SVG 可成功绘制，但 opaque
  parse failure、未明确的 fit 语义、DOM cache、资源限制和缺失 binary-cache 覆盖尚不能
  成为可靠公开契约。
- 增加默认关闭的 `svg-research` Cargo feature、本地 fixture、raster 断言和环境变量控制
  的 native overlay，使证据可以复验，同时不改变默认 dylib 或 Calcit API。
- 增加 path-scoped Linux workflow，使用精确 `setup-calcit v1.5.0` 记录 baseline/SVG
  release 体积，并执行 raster 与 Xvfb native probes。
- 处理 review 意见：将两个可移动 workflow action 引用替换为精确发布 tag（`checkout
  v7.0.1` 与 `setup-rust-toolchain v2.0.0`），并为涉及的绘制 helper 补充文档。按照仓库
  规则，release tag 继续作为版本 source of truth，不使用裸 commit hash 代理版本语义。
- 记录 arm64 macOS 能力、失败行为、尺寸语义、artifact 体积、构建观察、替代方案，以及
  重新考虑严格本地 SVG 资源 API 的明确门槛。
- Linux Actions run 34389883340 的 release raster test 与 Xvfb native overlay 均通过；
  实测 baseline 33,603,416 bytes、SVG 34,304,096 bytes（+700,680 / +2.09%），观察到
  的构建时间分别为 139.55 秒与 1,004.46 秒，证明该 SVG feature 组合缺少快速 binary
  路径。
