# Structured diagnostics for missing resources / 缺失资源的结构化诊断

## English

- Add `calcit-paint.core/check-resources!` (C-safe buffer v1 export `check_resources_calcit_ffi_v1`) which performs filesystem existence checks for `:image :file-path` and single-line `:text :font-file` after resource-root resolution.
- Return the same structured `PaintSceneDiagnostic` records as `validate-scene-structured`, with code `:missing-resource`, the root-relative shape `:path`, and the offending `:field`.
- Document that, unlike `validate-scene-structured`, this call reads the filesystem and belongs in explicit validation/tooling rather than every frame.
- Cover the traversal with a Rust unit test, a `check-examples` target, and a cookbook smoke assertion that a missing image yields exactly one diagnostic.
- Stacks on #111 (`codex/embedded-fonts2`) because the smoke exercises `:font-file`; retarget to `main` after #111 merges.

Validation:

- `calcit` canonical format zero diff; `--check-only`; quality baseline unchanged at 11 schema-dynamic positions; zero dynamic-method findings.
- README `format-md`/`check-md`; ten `check-examples` targets; `calcit test --require-match` (16 passed).
- `./scripts/check-cookbook.sh` checks valid scenes and detects a missing image; `check-creative-art.sh` hash unchanged; `check-actions-pins.py`.
- `cargo fmt --check`, 105 Rust tests, strict Clippy, release build.

## 中文

- 新增 `calcit-paint.core/check-resources!`（C-safe buffer v1 导出 `check_resources_calcit_ffi_v1`），在 resource-root 解析后对 `:image :file-path` 与单行 `:text :font-file` 做文件存在性检查。
- 返回与 `validate-scene-structured` 相同的结构化 `PaintSceneDiagnostic`：code 为 `:missing-resource`，包含根相对 shape `:path` 与出错 `:field`。
- 文档说明：与 `validate-scene-structured` 不同，此调用会访问文件系统，应放在显式校验/工具流程而非每帧。
- 覆盖：Rust 单元测试、`check-examples` target，以及 cookbook smoke 中断言缺失图片恰好产生 1 条诊断。
- 堆叠在 #111（`codex/embedded-fonts2`）之上，因为 smoke 使用了 `:font-file`；#111 合并后改回 `main`。

验证：

- `calcit` canonical format 零 diff；`--check-only`；quality baseline 维持 11 个 schema-dynamic 位置；动态方法零命中。
- README `format-md`/`check-md`；10 个 `check-examples` target；`calcit test --require-match`（16 passed）。
- `./scripts/check-cookbook.sh` 校验合法场景并检测缺失图片；`check-creative-art.sh` hash 不变；`check-actions-pins.py`。
- `cargo fmt --check`、105 个 Rust 测试、strict Clippy、release build。

Related: #106, #111.
