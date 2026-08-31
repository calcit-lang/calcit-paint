# Cookbook review follow-ups / Cookbook 评审跟进

## Review findings / 评审发现

- The typed-event recipe used non-existent aggregate tags and omitted required
  `WindowOptions` fields. The bilingual ordered lists were adjacent without a
  Markdown separator.
- typed-event recipe 使用了不存在的聚合 tag，并遗漏必填的 `WindowOptions` 字段；
  双语有序列表之间也缺少 Markdown 分隔。
- The cookbook smoke reused a repository-local PNG name and only checked its
  magic bytes, risking overwrite/leaks and failing to prove rendered pixels.
- cookbook smoke 复用了仓库内 PNG 名称且只检查 magic bytes，存在覆盖/残留风险，
  也不能证明实际绘制像素。

## Resolution / 解决

- Match real `PaintEvent` variants (`:mouse-down`, `:focus-in`, and
  `:accessibility-action`), read schema-backed payload fields, and construct a
  complete `WindowOptions`. Separate English and Chinese checklists with headings.
- 匹配真实 `PaintEvent` variant（`:mouse-down`、`:focus-in`、
  `:accessibility-action`），读取 schema 对应 payload 字段，并构造完整的
  `WindowOptions`；通过标题分隔中英文清单。
- Use a unique `mktemp` output with an `EXIT` cleanup trap. Decode the RGBA PNG
  in the smoke test and assert both transparent background and known red
  rectangle pixels, so failure and success both leave no artifact behind.
- 使用唯一的 `mktemp` 输出与 `EXIT` cleanup trap。smoke 会解码 RGBA PNG，并同时
  断言透明背景和已知红色矩形像素，因此成功或失败都不会遗留 artifact。
