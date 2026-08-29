# Text layout controls and measurement

## 中文

- 为 text shape 增加 `:font-family`、`:weight`、`:style` 与 `:baseline`，默认仍是系统默认字体、400、`:normal` 和 `:alphabetic`，因此旧数据无需修改。
- 字重限制在 100 至 900 的整数；为兼容现有 Snapshot，仍接受 `|300` 形式的数字字符串。非法字重、样式或基线会返回字段级错误。
- 通过 Skia `FontMgr` 按字体族、字重和斜体解析 typeface；目标字体未安装时回退到平台默认 typeface，并尽量保留字重与样式。
- 新增 `measure_text_calcit_ffi_v1` 与 `calcit-paint.core/measure-text!`。其返回 width、line-height、ascent、descent、leading 和 alphabetic baseline；空字符串宽度为零但仍保留行度量。
- 默认 `calcit.cirru` demo 在启动时实际调用测量 API，并在画布展示粗斜体/top、常规/middle、细体/bottom 的文本。README 同步提供中英双语契约和运行方式。

## English

- Added `:font-family`, `:weight`, `:style`, and `:baseline` to text shapes. Existing data remains valid because the defaults are the platform font, 400, `:normal`, and `:alphabetic`.
- Weights are integral from 100 through 900. Numeric string weights such as `|300` remain supported for the existing Snapshot; invalid weights, styles, and baselines produce field-specific errors.
- Skia `FontMgr` resolves family, weight, and slant. If a requested family is unavailable, rendering falls back to the platform default while preserving the requested style as closely as possible.
- Added `measure_text_calcit_ffi_v1` and `calcit-paint.core/measure-text!`. It returns width, line height, ascent, descent, leading, and the alphabetic baseline; empty text has zero width while retaining line metrics.
- The default `calcit.cirru` demo calls the measurement API at startup and draws bold-italic/top, regular/middle, and light/bottom text. The README documents the bilingual contract and runnable workflow.
