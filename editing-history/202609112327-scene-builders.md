# Opt-in scene builders / 可选场景构造辅助

## English

- Add the opt-in `calcit-paint.ui` namespace with transparent builders for common controls: `button`, `touch-container`, `text-input-focus-area`, and `icon-label`, plus `demo-scene`.
- Builders return the existing public scene-map DSL and flow through `validate-scene`, `render-canvas!`, and `render-to-png!` unchanged. `:action`/`:path`/`:data`, focus, and accessibility stay explicit and are never hidden.
- Introduce nominal option structs (`ButtonOptions`, `TouchContainerOptions`, `TextInputOptions`, `IconLabelOptions`) that prefer typed fields and omit unused optional values instead of writing `nil`.
- Cover every builder with a `calcit query examples`-discoverable validation example and a definition-attached test; `demo-scene` composes all four builders and is validated plus rendered by `./scripts/check-cookbook.sh`.
- Document the builders in the bilingual README and `docs/cookbook.md`, add the `docs/architectures/scene-builders.cirru` scaffold, and extend the CI `check-examples` step.
- Rebase `config/calcit-quality.json` from 6 to 11 schema-dynamic positions: the five builders intentionally expose the open `Map<Tag, Dynamic>` scene boundary, matching the existing `build-art-scene` contract. No new `codeNil`, `codeDynamic`, or `unsafeCoerce` debt is added.

Validation:

- `calcit ./calcit.cirru edit format` canonical with zero unintended diff and `--check-only`
- reviewed `analyze quality --baseline config/calcit-quality.json` and zero dynamic-method findings
- `calcit docs format-md`/`check-md` for README and cookbook, plus all eight `check-examples` targets
- `calcit test` (14 passed), `./scripts/check-cookbook.sh`, and `./scripts/check-creative-art.sh`
- `cargo fmt --check`, 97 Rust tests, and strict Clippy

## 中文

- 新增 opt-in `calcit-paint.ui` namespace，提供透明常用控件 builder：`button`、`touch-container`、`text-input-focus-area`、`icon-label`，以及 `demo-scene`。
- builder 返回既有公开 scene-map DSL，可原样通过 `validate-scene`、`render-canvas!` 与 `render-to-png!`。`:action`/`:path`/`:data`、focus 与无障碍语义保持显式，不会被隐藏。
- 引入 nominal 选项 struct（`ButtonOptions`、`TouchContainerOptions`、`TextInputOptions`、`IconLabelOptions`），优先使用强类型字段，并在缺省时省略可选值而不是写入 `nil`。
- 每个 builder 都有 `calcit query examples` 可发现的校验示例与 definition-attached test；`demo-scene` 组合四个 builder，并由 `./scripts/check-cookbook.sh` 完成校验与绘制。
- 在双语 README 与 `docs/cookbook.md` 记录 builder，新增 `docs/architectures/scene-builders.cirru` scaffold，并扩展 CI `check-examples` step。
- 将 `config/calcit-quality.json` 从 6 更新为 11 个 schema-dynamic 位置：五个 builder 有意暴露开放的 `Map<Tag, Dynamic>` 场景边界，与既有 `build-art-scene` 契约一致；未新增 `codeNil`、`codeDynamic` 或 `unsafeCoerce` 债务。

验证：

- `calcit ./calcit.cirru edit format` canonical 零意外 diff，并通过 `--check-only`
- 已审核的 `analyze quality --baseline config/calcit-quality.json` 与零动态方法命中
- README 与 cookbook 的 `calcit docs format-md`/`check-md`，以及全部 8 个 `check-examples` target
- `calcit test`（14 passed）、`./scripts/check-cookbook.sh` 与 `./scripts/check-creative-art.sh`
- `cargo fmt --check`、97 个 Rust 测试与 strict Clippy

Related: #87, #45, #969 (upstream Calcit `fs-path` blocker tracked for the deferred 0.14.7 upgrade).
