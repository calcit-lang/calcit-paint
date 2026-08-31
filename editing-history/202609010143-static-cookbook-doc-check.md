# Cookbook static documentation checks / Cookbook 静态文档检查

The documentation checker distinguishes three fence modes: `cirru` runs a
snippet, `cirru.no-run` parses and preprocesses it without evaluation, and
`cirru.no-check` only parses it. Native-FFI recipes that must not open a window
or write files during markdown validation should therefore use
`cirru.no-run`, not `cirru.no-check`.

文档检查器区分三种 fence 模式：`cirru` 会运行片段，`cirru.no-run` 会解析并预处理
但不求值，`cirru.no-check` 只解析。Markdown 校验期间不能打开窗口或写文件、但仍需
验证 native FFI recipe 的场景，应使用 `cirru.no-run`，而不是 `cirru.no-check`。

Each cookbook snippet keeps an explicit `ns` declaration so `calcit docs
check-md docs/cookbook.md --entry calcit.cirru` can resolve the imported public
API. Native behavior is separately covered by `scripts/check-cookbook.sh`.

每个 cookbook 片段保留显式 `ns` 声明，使
`calcit docs check-md docs/cookbook.md --entry calcit.cirru` 能解析所导入的公开
API。原生行为由 `scripts/check-cookbook.sh` 单独覆盖。
