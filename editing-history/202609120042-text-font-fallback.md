# Per-character text font fallback / 单行文本逐字符字体回退

## English

- Fix single-line `:text` rendering so characters missing from the resolved font (for example CJK or Arabic in a Latin face) use an installed fallback typeface instead of rendering tofu boxes.
- Split text into consecutive runs that share one resolved typeface. Each run is drawn with its own `Font`; alignment and width use the summed run widths.
- Resolve fallback typefaces through `FontMgr::match_family_style_character` with `zh-Hans`, `zh-Hant`, `ja`, `ko`, and `ar` language hints, caching fonts by typeface identity.
- Keep the requested weight and slant for every run, and keep the existing baseline metrics from the primary font so `:top`/`:middle`/`:bottom`/`:alphabetic` behavior is unchanged.
- Make `measure-text!` fallback-aware so measured width matches drawn width for mixed-script text.
- Add a platform-independent Rust test that runs are text-preserving, non-empty for mixed input, and measurable.

Validation:

- `cargo fmt --check`, `cargo test` (98 passed), and strict Clippy
- offscreen render of `Regenerated / 已重新生成`, `English 中文 日本語 العربية`, and `按 P 导出 · 已导出` shows real glyphs instead of boxes
- `./scripts/check-creative-art.sh` deterministic PNG hash unchanged
- `calcit` snapshot format/check-only/quality/docs/tests unchanged

## 中文

- 修复单行 `:text` 渲染：当解析出的字体缺少某些字符（例如拉丁字体中的中日韩或阿拉伯字符）时，改用系统中已安装的 fallback 字体，而不是显示方框。
- 将文本拆分为共用同一 typeface 的连续 run，每个 run 用自己的 `Font` 绘制；对齐与宽度使用各 run 宽度之和。
- 通过 `FontMgr::match_family_style_character` 并带 `zh-Hans`、`zh-Hant`、`ja`、`ko`、`ar` 语言提示解析 fallback typeface，并按 typeface 身份缓存字体。
- 每个 run 保留请求的字重与斜体；基线度量继续取自主字体，因此 `:top`/`:middle`/`:bottom`/`:alphabetic` 行为不变。
- `measure-text!` 也会计入 fallback run，使混合文字场景下测量宽度与实际绘制一致。
- 新增与平台无关的 Rust 测试：run 保留原文、混合输入非空、宽度可测。

验证：

- `cargo fmt --check`、`cargo test`（98 passed）与 strict Clippy
- 离屏渲染 `Regenerated / 已重新生成`、`English 中文 日本語 العربية`、`按 P 导出 · 已导出` 显示真实字形而非方框
- `./scripts/check-creative-art.sh` 确定性 PNG hash 不变
- `calcit` snapshot format/check-only/quality/docs/tests 均不受影响

Related: reported through the Creative Art Preview demo; complements the paragraph ICU/BiDi shaping already used by `:paragraph`.
