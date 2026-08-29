# Paragraph layout and international text / 段落布局与国际化文本

## 中文

- 启用 rust-skia 的 `textlayout` feature，直接使用 Skia Paragraph、ICU 与 BiDi shaping，
  不增加按 UTF-8 字节切分的降级实现。
- 新增 `:paragraph` shape（兼容别名 `:text-block`），支持显式换行、最大宽度、绝对行高、
  左中右对齐、LTR/RTL、最大行数与字符串省略号；现有单行 `:text` 保持不变。
- 绘制与 `measure-paragraph!` 共用同一段落 builder。测量 API 返回同构的
  `Map<Tag, Number>`，包含尺寸、行数、内在宽度与基线。
- 默认 Calcit demo 实际绘制中英文显式换行、两行省略和阿拉伯文 RTL 三组段落，并在
  启动窗口前打印段落测量结果。
- 新增空文本、CJK、RTL、显式换行、最大行数、省略号、非法宽度/行高以及绘制/测量一致性测试。
- C-safe export 审计加入单行与段落两个测量入口；类型质量基线只增加段落选项异构 map 这一处
  已审核的 Dynamic 边界。

## English

- Enabled rust-skia's `textlayout` feature and use Skia Paragraph with ICU/BiDi shaping directly,
  without a UTF-8 byte-slicing fallback.
- Added the `:paragraph` shape and `:text-block` alias with explicit newlines, maximum width,
  absolute line height, left/center/right alignment, LTR/RTL direction, maximum lines, and a string
  ellipsis. Existing single-line `:text` behavior is unchanged.
- Drawing and `measure-paragraph!` share one paragraph builder. The measurement API returns a
  homogeneous `Map<Tag, Number>` with dimensions, line count, intrinsic widths, and baselines.
- The default runnable Calcit demo renders Chinese/English explicit newlines, two-line ellipsis,
  and Arabic RTL paragraphs, and prints paragraph metrics before opening the window.
- Added tests for empty text, CJK, RTL, explicit newlines, maximum lines, ellipsis, invalid width and
  line height, plus drawing/measurement consistency.
- Extended the C-safe export audit for both measurement entry points. The reviewed type baseline
  adds only the genuine heterogeneous paragraph-options map boundary.

## Verification / 验证

- `cargo fmt --check`
- `cargo test` (30 passed)
- `cargo clippy --all-targets -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README example check, quality baseline, and dynamic-method gate
- `./build.sh` and real macOS `CALCIT_PAINT_SMOKE_ONCE=1` default-demo smoke
