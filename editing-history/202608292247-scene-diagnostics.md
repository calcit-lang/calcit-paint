# Strict scene validation and path-aware diagnostics / 严格场景校验与路径诊断

## 中文

- 新增 `validate-scene` Calcit API 与 `validate_scene_calcit_ffi_v1` C-safe buffer 导出，返回收紧的 `List<String>`；合法 scene 返回空列表。
- shape decoder 现在递归携带稳定结构路径（`$`、`$.children[n]`），并聚合同级多个非法节点，不再用空 group 静默替换失败节点。
- 窗口绘制、`render-to-png!` 与显式校验共用同一套严格解析；窗口错误写入 stderr，未知 drawing op 返回错误，不再污染 stdout。
- 离屏回归覆盖非法 scene 不会生成半成品 PNG；Rust tests 还覆盖合法根节点、根错误、嵌套路径、多个 sibling 错误和非法 children 类型。
- 默认 `calcit.cirru` 新增并实际运行 `validate-scene-demo!`：断言合法 scene 无诊断、非法 scene 恰好产生两条嵌套诊断，再正常打开窗口。
- README 与 GitHub Actions 导出符号审计同步更新；路线图 #45 已整理，后续按 #50 → #51 → #52 推进。

## English

- Added the Calcit `validate-scene` API and the C-safe `validate_scene_calcit_ffi_v1` buffer export with a strict `List<String>` result; valid scenes return an empty list.
- The shape decoder now carries stable structural paths (`$` and `$.children[n]`) recursively and aggregates multiple invalid siblings instead of silently replacing failures with empty groups.
- Window drawing, `render-to-png!`, and explicit validation share the same strict parser. Window failures go to stderr, and unknown drawing operations return errors instead of contaminating stdout.
- Offscreen regression coverage proves an invalid scene does not leave a partial PNG. Rust tests also cover valid roots, root failures, nested paths, multiple sibling failures, and invalid children containers.
- The default `calcit.cirru` now includes and actually runs `validate-scene-demo!`: it asserts no diagnostics for a valid scene and exactly two nested diagnostics for an invalid scene before opening the normal window.
- Updated the bilingual README and the GitHub Actions export audit. Roadmap #45 is organized for the #50 → #51 → #52 sequence.

## Verification / 验证

- `cargo fmt --all -- --check`
- `cargo test` (43 passed)
- `cargo clippy --all-targets --all-features -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 Snapshot check, README check-md, quality baseline, and dynamic-method gate
- Release `./build.sh`, macOS exported-symbol audit, and a real auto-exit Calcit window smoke
