# Per-character text font fallback / 单行文本逐字符字体回退

## English

- Fix single-line `:text` rendering so characters missing from the resolved font (for example CJK or Arabic in a Latin face) use an installed fallback typeface when an installed fallback matches, instead of rendering tofu boxes. Characters with no matching installed fallback keep the primary face and may still show a tofu box.
- Split text into consecutive grapheme-cluster runs that share one resolved typeface. Each cluster is resolved as a unit so combining marks and ZWJ sequences are never split across typefaces. Each run is drawn with its own `Font`; alignment and width use the summed run widths.
- Resolve fallback typefaces through `FontMgr::match_family_style_character` with BCP47 hints, caching fonts by typeface identity. The hint list keeps the project's Simplified Chinese default but appends an optional `:lang` tag last, which Skia treats as the most significant hint.
- Keep the requested weight and slant for every run, and keep the existing baseline metrics from the primary font so `:top`/`:middle`/`:bottom`/`:alphabetic` behavior is unchanged.
- Make `measure-text!` fallback-aware so measured width matches drawn width for mixed-script text.
- Add platform-independent Rust tests: runs are text-preserving, mixed input is non-empty and measurable, and a combining-mark cluster stays in one run.

Note: single-line `:text` still uses Skia's simple `TextBlob` path and does not perform complex shaping; Arabic joining and emoji composition remain the job of `:paragraph` (Skia Paragraph/ICU). This change only adds per-cluster font fallback and does not reorder text.

Validation:

- `cargo fmt --check`, `cargo test` (98 passed), and strict Clippy
- offscreen render of `Regenerated / 已重新生成`, `English 中文 日本語 العربية`, and `按 P 导出 · 已导出` shows real glyphs instead of boxes
- `./scripts/check-creative-art.sh` deterministic PNG hash unchanged
- `calcit` snapshot format/check-only/quality/docs/tests unchanged

## 中文

- 修复单行 `:text` 渲染：当解析出的字体缺少某些字符（例如拉丁字体中的中日韩或阿拉伯字符）时，只有在有已安装 fallback 匹配时才改用该字体，而不是显示方框。若系统中没有匹配的 fallback，字符仍使用主字体，可能显示方框。
- 将文本按 grapheme cluster 拆分为共用同一 typeface 的连续 run；每个 cluster 作为整体解析，避免组合字符与 ZWJ 序列被拆到不同字体。每个 run 用自己的 `Font` 绘制；对齐与宽度使用各 run 宽度之和。
- 通过 `FontMgr::match_family_style_character` 并带 BCP47 提示解析 fallback typeface，并按 typeface 身份缓存字体。提示列表保留项目的简体中文默认，并把可选的 `:lang` 标签追加在最后；Skia 会把最后一项视为最高优先级提示。
- 每个 run 保留请求的字重与斜体；基线度量继续取自主字体，因此 `:top`/`:middle`/`:bottom`/`:alphabetic` 行为不变。
- `measure-text!` 也会计入 fallback run，使混合文字场景下测量宽度与实际绘制一致。
- 新增与平台无关的 Rust 测试：run 保留原文、混合输入非空且宽度可测、组合字符 cluster 不会被拆分。

说明：单行 `:text` 仍使用 Skia 的简单 `TextBlob` 路径，不做复杂 shaping；阿拉伯连写与 emoji 组合仍由 `:paragraph`（Skia Paragraph/ICU）负责。本改动只增加按 cluster 的字体回退，不改变文字顺序。

验证：

- `cargo fmt --check`、`cargo test`（98 passed）与 strict Clippy
- 离屏渲染 `Regenerated / 已重新生成`、`English 中文 日本語 العربية`、`按 P 导出 · 已导出` 显示真实字形而非方框
- `./scripts/check-creative-art.sh` 确定性 PNG hash 不变
- `calcit` snapshot format/check-only/quality/docs/tests 均不受影响

Related: reported through the Creative Art Preview demo; complements the paragraph ICU/BiDi shaping already used by `:paragraph`.
