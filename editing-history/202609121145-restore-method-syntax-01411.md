# Restore Option/Result method syntax on Calcit 0.14.11 / 在 Calcit 0.14.11 上回归 Option/Result 方法调用

## English

- Upgrade calcit-paint's declared Calcit runtime baseline from 0.14.10 to 0.14.11, keeping module version `0.2.0`, `calcit_native_ffi 0.1.2`, the buffer/blocking-callback v1 ABI, and public APIs unchanged.
- Restore receiver-method syntax in typed code now that calcit-lang/calcit#985 (nominal Option/Result methods, released in 0.14.11) makes `.unwrap-or`, `.unwrap`, and `.some?` statically specializable again. The temporary `option:unwrap-or` / `option:unwrap` / `option:some?` forms from the 0.14.10 migration are gone.
- Keep `&merge` / `&list:concat` for the documented open scene-map boundary. `calcit.core/merge-dynamic` now exists (#986 / 0.14.11), but it still rejects empty maps produced by `({})` with `E_ERASED_GENERIC_RELATION`, and the `assert-type` workaround miscompiles inside a multi-line `if` branch; both are reported as calcit-lang/calcit#992. Using `&merge` / `&list:concat` keeps the builders correct until that is fixed.
- `calcit.cirru` stays canonical with zero unintended churn; `config/calcit-quality.json` is unchanged at 11 schema-dynamic positions.

Validation with Calcit 0.14.11:

- `caps --strict --ci` with no version warning
- canonical Snapshot formatting with zero diff and `calcit ./calcit.cirru --check-only`
- `analyze quality --baseline config/calcit-quality.json` PASS and zero dynamic-method findings
- `docs format-md`/`check-md` for README and cookbook, and all eight `check-examples` targets
- `calcit test --require-match` (16 passed)
- `./scripts/check-cookbook.sh` and `./scripts/check-creative-art.sh` (deterministic hash unchanged)
- `cargo fmt --check`, 102 Rust tests, strict Clippy, release build, and the Actions provenance audit

## 中文

- 将 calcit-paint 声明的 Calcit 运行时基线从 0.14.10 升级到 0.14.11，保持模块版本 `0.2.0`、`calcit_native_ffi 0.1.2`、buffer/blocking-callback v1 ABI 与公开 API 不变。
- 随着 calcit-lang/calcit#985（nominal Option/Result 方法，随 0.14.11 发布）落地，typed 代码可以重新使用接收者方法 `.unwrap-or`、`.unwrap`、`.some?` 并可静态专门化；0.14.10 迁移期临时使用的 `option:unwrap-or` / `option:unwrap` / `option:some?` 已全部移除。
- 开放 scene-map 边界继续使用 `&merge` / `&list:concat`。`calcit.core/merge-dynamic` 虽已随 #986 发布（0.14.11），但由 `({})` 产生的空 map 仍会触发 `E_ERASED_GENERIC_RELATION`，而 `assert-type` 规避写法在多行 `if` 分支中会被误编译；二者已上报为 calcit-lang/calcit#992。在修复前保留 `&merge` / `&list:concat` 以保证 builder 正确。
- `calcit.cirru` 保持 canonical 且无意外 churn；`config/calcit-quality.json` 维持 11 个 schema-dynamic 位置不变。

使用 Calcit 0.14.11 验证：

- `caps --strict --ci` 且无版本 warning
- canonical Snapshot 格式化零 diff，以及 `calcit ./calcit.cirru --check-only`
- `analyze quality --baseline config/calcit-quality.json` PASS 且动态方法零命中
- README/cookbook 的 `docs format-md`/`check-md`，以及全部 8 个 `check-examples` target
- `calcit test --require-match`（16 passed）
- `./scripts/check-cookbook.sh` 与 `./scripts/check-creative-art.sh`（确定性 hash 不变）
- `cargo fmt --check`、102 个 Rust 测试、strict Clippy、release build 与 Actions 来源审计

Related: calcit-paint#103, calcit-lang/calcit#985, calcit-lang/calcit#986, calcit-lang/calcit#992.
