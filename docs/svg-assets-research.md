# SVG assets for 0.1.0 / 0.1.0 SVG 资源调研

## Decision / 结论

Calcit Paint will **not expose a public SVG scene node in 0.1.0**. Skia SVG is
technically usable and the measured linked-size increase is modest, but the
current parser exposes no actionable failure detail, absolute-size and
`viewBox` behavior needs an explicit fit policy, and a production API would
need bounded loading plus a revision-aware DOM cache. Paths and local raster
images already cover the Creative Art Preview milestone without committing to
an underspecified resource contract.

Calcit Paint **不会在 0.1.0 公开 SVG scene node**。Skia SVG 技术上可用，实测链接体积
增幅也较小；但当前 parser 不提供可行动的失败细节，绝对尺寸与 `viewBox` 行为需要明确
fit 策略，正式 API 还需要有界加载和 revision-aware DOM cache。现有 path 与本地 raster
image 已能完成 Creative Art Preview milestone，无需过早承诺一个定义不足的资源契约。

The non-default `svg-research` Cargo feature and its fixture are an internal,
repeatable evidence harness. They are not a Calcit API and do not change the
default dylib. / 默认关闭的 `svg-research` Cargo feature 与 fixture 只用于内部可复验
证据；它们不是 Calcit API，也不改变默认 dylib。

## Current capability / 当前能力

The project pins `skia-safe 0.97.2` (Skia `m148`). Its `svg` feature enables
`skia-bindings/svg`, `skia-svg-macros`, `base64`, and `percent-encoding`;
`skia-bindings/svg` also requires textlayout, which Paint already enables.
No DOM, CSS, JavaScript, remote-URL, or WebGPU layer is involved.

项目锁定 `skia-safe 0.97.2`（Skia `m148`）。其 `svg` feature 会启用
`skia-bindings/svg`、`skia-svg-macros`、`base64` 和 `percent-encoding`；
`skia-bindings/svg` 还依赖 Paint 已启用的 textlayout。本调研不涉及 DOM、CSS、
JavaScript、远程 URL 或 WebGPU 层。

`skia_safe::svg::Dom` can parse bytes/strings with a resource provider and
render into any Skia `Canvas`. The research harness deliberately uses
`FontMgr::empty()` and does not enable the optional `ureq` provider, so it does
not acquire remote resources. A future API must keep that boundary explicit.

`skia_safe::svg::Dom` 可通过 resource provider 解析 bytes/string，并绘制到任意 Skia
`Canvas`。research harness 有意使用 `FontMgr::empty()`，且不启用可选的 `ureq`
provider，因此不会获取远程资源。未来若开放 API，必须继续显式保持该边界。

## Fixture evidence / Fixture 证据

The checked fixture combines a `48×32` intrinsic size, `0 0 24 16` viewBox,
gradient, clip path, rectangle, and path. The harness verifies:

- parsing succeeds and exposes the expected intrinsic size and viewBox;
- malformed XML is rejected;
- a `192×128` destination renders non-background pixels into a raster surface
  and produces a valid PNG signature;
- the same renderer can be injected into the existing one-shot native OpenGL
  window via `CALCIT_PAINT_SVG_RESEARCH=1` when the research feature is built.

受检 fixture 组合了 `48×32` intrinsic size、`0 0 24 16` viewBox、gradient、clip
path、rectangle 与 path。harness 会验证：

- 能成功解析，并暴露预期 intrinsic size 与 viewBox；
- 非法 XML 会被拒绝；
- `192×128` 目标会在 raster surface 中产生非背景像素及合法 PNG signature；
- 构建 research feature 后，可通过 `CALCIT_PAINT_SVG_RESEARCH=1` 将同一 renderer
  注入既有 one-shot native OpenGL window。

Skia does not scale an SVG with absolute root width/height merely because
`set_container_size` changes. The harness therefore applies an explicit
contain transform. Any public shape must define `fill`/`contain`/`cover`, crop,
alignment, and pixel-density semantics rather than leaking this detail.

对于使用绝对 root width/height 的 SVG，仅调用 `set_container_size` 不会自动缩放内容；
harness 因而显式应用 contain transform。未来公开 shape 必须定义 `fill`/`contain`/
`cover`、crop、alignment 与 pixel-density 语义，而不是泄漏该实现细节。

The current `LoadError` is always `Failed to load svg (reason unknown)`. It can
distinguish success from failure but cannot identify an element, attribute,
line, external resource, or repair. Wrapping that string in Paint structured
diagnostics would not make it actionable.

当前 `LoadError` 始终为 `Failed to load svg (reason unknown)`。它只能区分成功与失败，
无法指出 element、attribute、line、外部资源或修复方式；即使包装为 Paint 结构化诊断，
也不会因此变得可行动。

## Measurements / 测量

Local measurements used arm64 macOS, Rust 1.94.0, the same main commit, release
mode, fresh isolated target directories, and source builds after both expected
rust-skia binary-cache URLs returned HTTP 404.

本地测量环境为 arm64 macOS、Rust 1.94.0、同一 main commit、release mode 与全新隔离
target directory；两个预期 rust-skia binary-cache URL 均返回 HTTP 404，因而实际执行
源码构建。

| macOS evidence / macOS 证据 | Baseline | SVG research | Delta / 差异 |
| --- | ---: | ---: | ---: |
| Skia ninja targets | 1,617 | 1,666 | +49 / +3.0% |
| Linked dylib bytes | 21,721,440 | 22,303,456 | +582,016 / +2.68% |
| Observed build wall time | 146 s after a partial failed attempt | 217 s clean | indicative only / 仅供参考 |

The timing rows are not a benchmark: the baseline reused Rust work completed
before a sandboxed network attempt failed, while the SVG target was clean.
They establish that missing binary-cache coverage can force a multi-minute
Skia source build; they do not claim a precise percentage regression.

时间行不是 benchmark：baseline 复用了 sandbox network 尝试失败前已完成的 Rust 工作，
SVG target 则为全新构建。它只证明缺失 binary-cache 覆盖会触发数分钟 Skia 源码构建，
不用于宣称精确的百分比回归。

One macOS offscreen probe produced a `192×128` RGBA PNG of 1,331 bytes with
SHA-256 `4df3b67fd8ca6a9916cdd087f98cbed4972bd7d71a9edf443adac65eee3c4fae`.
The native research overlay completed the one-shot Creative Art window in
2.04 seconds.

一次 macOS 离屏 probe 产生 `192×128` RGBA PNG，大小 1,331 bytes，SHA-256 为
`4df3b67fd8ca6a9916cdd087f98cbed4972bd7d71a9edf443adac65eee3c4fae`。
native research overlay 在 2.04 秒内完成 one-shot Creative Art window。Linux 的
release、raster 与 Xvfb/native 证据由 path-scoped `SVG Research` workflow 生成。

The first [Linux SVG Research run](https://github.com/calcit-lang/calcit-paint/actions/runs/34389883340)
passed both the raster assertion and the Xvfb OpenGL-window overlay:

首次 [Linux SVG Research run](https://github.com/calcit-lang/calcit-paint/actions/runs/34389883340)
同时通过 raster 断言和 Xvfb OpenGL-window overlay：

| Linux evidence / Linux 证据 | Baseline | SVG research | Delta / 差异 |
| --- | ---: | ---: | ---: |
| Linked `.so` bytes | 33,603,416 | 34,304,096 | +700,680 / +2.09% |
| Observed release build wall time | 139.55 s | 1,004.46 s | cache-path evidence / cache 路径证据 |
| One-shot native window | existing Test workflow passed | 2.674 s, passed | SVG overlay exercised / 已绘制 SVG overlay |

The SVG step ran after the baseline in the same job yet took about 16.7
minutes, consistent with the SVG feature combination missing the fast binary
path and compiling a second Skia configuration. This is operational evidence,
not a stable performance ratio; runner load and future cache coverage can
change it.

SVG step 在同一 job 的 baseline 之后执行，却仍耗时约 16.7 分钟，符合 SVG feature
组合缺少快速 binary 路径、需要编译第二套 Skia 配置的表现。这是运维证据而非稳定性能
比例；runner 负载与未来 cache 覆盖都可能改变结果。

## Reproduce / 复验

```bash
cargo test --release --features svg-research svg_research
cargo build --release --features svg-research
```

After copying the feature-enabled dylib into `dylibs/`, exercise the real
window with the project-pinned Calcit runtime:

将 feature-enabled dylib 复制到 `dylibs/` 后，用项目锁定的 Calcit runtime 验证真实窗口：

```bash
CALCIT_PAINT_SMOKE_ONCE=1 CALCIT_PAINT_SVG_RESEARCH=1 calcit ./calcit.cirru
```

## Revisit gate / 重启实现的门槛

A later implementation issue should be opened only when a real product scene
needs SVG and can accept all of these contracts:

- local bytes/path only, bounded file and decoded-resource sizes, no remote URL;
- stable `invalid-svg`, `missing-resource`, and size/fit diagnostics before the
  first window frame;
- revision-aware DOM/resource cache with an explicit invalidation key;
- documented fit, alignment, crop, density, text/font, and unsupported-feature
  behavior;
- maintained rust-skia binary-cache coverage or an accepted source-build cost;
- runnable Calcit demo plus macOS and Linux raster/native smoke coverage.

只有实际产品 scene 需要 SVG，并能接受以下全部契约时，才应创建后续实现 issue：

- 仅允许本地 bytes/path，限制文件与解码资源大小，不支持远程 URL；
- 首帧窗口前提供稳定的 `invalid-svg`、`missing-resource` 与 size/fit 诊断；
- 带显式 invalidation key 的 revision-aware DOM/resource cache；
- 记录 fit、alignment、crop、density、text/font 和 unsupported-feature 行为；
- 持续维护 rust-skia binary-cache 覆盖，或明确接受源码构建成本；
- 提供可运行 Calcit demo，以及 macOS/Linux raster/native smoke。

Until then, authors should use Paint path operations for scalable procedural
art and local PNG/JPEG images for authored assets. / 在满足这些门槛前，开发者应使用
Paint path operation 制作可缩放程序化图形，并使用本地 PNG/JPEG 承载制作完成的资源。
