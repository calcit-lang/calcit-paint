# Embedded font files for single-line text / 单行文本的内嵌字体文件

## English

- Add an optional `:font-file` field to single-line `:text` and `measure-text!` options: the renderer loads a typeface directly from the file and uses it as the primary face.
- Font paths resolve through the resource root (from the resource-path convention), so bundled fonts ship alongside images. Typefaces are cached by resolved path.
- A missing or undecodable font file logs a diagnostic and falls back to the existing family/default resolution; missing glyphs still fall back per grapheme cluster through Skia.
- `:paragraph` intentionally keeps family-based layout; embedded font files are not applied to paragraph text yet.
- Cover the extractor (`:font-file` parsing), the loader (bundled `resources/SourceCodePro-Medium.ttf`, missing-file fallback), and the runnable cookbook smoke (a text scene rendered with the bundled font).

Validation:

- `calcit` canonical format zero diff; `--check-only`; quality baseline unchanged at 11 schema-dynamic positions; zero dynamic-method findings.
- README `format-md`/`check-md`; `calcit test --require-match` (16 passed); `./scripts/check-cookbook.sh` renders the bundled-font text; `./scripts/check-creative-art.sh` hash unchanged.
- `cargo fmt --check`, 104 Rust tests, strict Clippy, release build, and the Actions provenance audit.

## 中文

- 为单行 `:text` 与 `measure-text!` 选项新增可选 `:font-file`：渲染器直接从文件加载 typeface 并作为主字体。
- 字体路径经 resource root 解析，使随包字体与图片使用同一约定。typeface 按解析后的路径缓存。
- 字体文件缺失或无法解码时记录诊断并回退到既有字体族/默认解析；缺失字符仍按 grapheme cluster 通过 Skia 回退。
- `:paragraph` 仍保持字体族布局，暂不应用内嵌字体文件。
- 覆盖 extracter（`:font-file` 解析）、loader（随包 `resources/SourceCodePro-Medium.ttf`、缺失文件回退）以及可运行 cookbook smoke（用随包字体渲染文本场景）。

验证：

- `calcit` canonical format 零 diff；`--check-only`；quality baseline 维持 11 个 schema-dynamic 位置；动态方法零命中。
- README `format-md`/`check-md`；`calcit test --require-match`（16 passed）；`./scripts/check-cookbook.sh` 渲染随包字体文本；`./scripts/check-creative-art.sh` hash 不变。
- `cargo fmt --check`、104 个 Rust 测试、strict Clippy、release build 与 Actions 来源审计。

Related: #106, #110.
