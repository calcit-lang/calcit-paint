# Structured scene diagnostics / 结构化场景诊断

## English

- Add the typed `PaintSceneDiagnostic` record and
  `validate-scene-structured`, exposing stable scene paths and diagnostic codes
  plus optional fields, expected/actual values, and human-readable messages.
- Preserve `validate-scene` as the compatible string API, derived from the same
  diagnostic records, and keep validation read-only across window and
  offscreen rendering paths.
- Cover nested children, invalid effect parameters, unsupported resource
  options, legacy/structured field conflicts, and interactive cached groups in
  Rust assertions and a runnable Calcit repair-loop example.
- Validate the generated Creative Art scene through the new public boundary
  before launching the default preview.
- Add bilingual Agent/developer troubleshooting guidance, a checked cookbook
  recipe, a declarative architecture contract, CI example execution, and C-safe
  export auditing.
- Isolate native dependency installation from the unrelated Chrome apt source
  preinstalled on GitHub-hosted runners, after its transient hash mismatch
  blocked the first PR run before repository code was executed.

Validation used the project-pinned Calcit 0.13.77:

- strict Caps resolution, canonical Snapshot formatting, `--check-only`, nine
  Calcit tests, the reviewed quality baseline, and zero dynamic-method findings
- executable examples for the compatible validator, structured validator, and
  Creative Art scene builder
- README/cookbook `format-md` and `check-md`, executable cookbook smoke, and
  deterministic 960x720 Creative Art PNG smoke
- real macOS blocking-window smoke with `CALCIT_PAINT_SMOKE_ONCE=1`
- `cargo fmt --check`, 97 Rust tests, strict Clippy, release dylib build, and
  audit of both scene-validation v1 symbols

## 中文

- 新增强类型 `PaintSceneDiagnostic` 记录与 `validate-scene-structured`，公开稳定的
  scene path 与诊断 code，并提供可选 field、expected/actual 和面向用户的 message。
- 保留 `validate-scene` 作为兼容字符串 API，并由同一组诊断记录派生；窗口和离屏绘制
  路径中的校验继续保持只读。
- 通过 Rust 断言与可运行 Calcit 修复闭环示例覆盖嵌套 children、非法 effect 参数、
  不受支持的资源选项、新旧字段冲突以及包含交互节点的 cached group。
- 默认 Creative Art 预览启动前，先通过新的公共边界校验生成的 scene。
- 增加双语 Agent/开发者排障说明、静态检查 cookbook recipe、声明式架构契约、CI
  example 执行和 C-safe 导出审计。
- GitHub hosted runner 预装的 Chrome apt source 出现瞬时 hash mismatch，并在仓库代码
  运行前阻断首次 PR Actions；native 依赖安装现与该无关源隔离。

验证使用项目锁定的 Calcit 0.13.77：

- strict Caps 解析、canonical Snapshot 格式、`--check-only`、九个 Calcit 测试、已审阅
  quality baseline 与动态 method 零命中
- 兼容 validator、结构化 validator 与 Creative Art scene builder 的可执行 examples
- README/cookbook 的 `format-md` 与 `check-md`、可执行 cookbook smoke，以及确定性
  960x720 Creative Art PNG smoke
- 使用 `CALCIT_PAINT_SMOKE_ONCE=1` 的真实 macOS blocking-window smoke
- `cargo fmt --check`、97 个 Rust 测试、strict Clippy、release dylib 构建，以及两个 scene
  validation v1 符号审计
