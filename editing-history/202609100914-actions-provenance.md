# Immutable Actions provenance / Actions 不可变来源

## English

- Follow up on the remaining security finding from PR #95 under issue #96.
- Pin every third-party GitHub Action to the verified 40-character commit
  behind its exact published release tag. Keep the tag beside the SHA so the
  human-readable version remains the source of version semantics while the SHA
  supplies immutable revision and integrity evidence.
- Standardize all workflows on `actions/checkout` v7.0.1,
  `actions-rust-lang/setup-rust-toolchain` v2.0.0, and
  `calcit-lang/setup-calcit` v1.5.0 where Calcit is required. Remove the broad
  `v4`, `v1`, and `stable` references.
- Grant only read access to repository contents and prevent checkout from
  persisting credentials.
- Add a repository-owned static audit and run it from the regular Test workflow
  so later workflow changes cannot silently reintroduce mutable references or
  omit the exact release-tag annotation.
- Rechecked the merged PR #94 source: `cargo fmt --check` passed and Calcit
  0.13.77 reported no canonical formatting changes for `calcit.cirru`.
- Validation passed for the provenance audit, all three workflow YAML files,
  strict Caps resolution, Calcit check/quality/dynamic-method gates, both
  statically checked documentation files, 97 Rust tests, strict Clippy, and the
  release dylib build.

## 中文

- 在 issue #96 中跟进 PR #95 遗留的安全 review 意见。
- 将所有第三方 GitHub Action 固定到精确已发布 tag 对应的 40 位
  commit SHA。SHA 旁保留 tag，tag 继续承载可读的版本语义，SHA 提供不可变
  revision 和完整性证据。
- 全部 workflow 统一使用 `actions/checkout` v7.0.1、
  `actions-rust-lang/setup-rust-toolchain` v2.0.0，需要 Calcit 时使用
  `calcit-lang/setup-calcit` v1.5.0，移除宽泛的 `v4`、`v1` 和 `stable`。
- 仓库权限收紧为只读 contents，checkout 不持久化凭据。
- 新增仓库自有静态审计，并纳入常规 Test workflow，防止后续修改重新引入
  可移动引用或遗漏精确 release tag 注释。
- 复核已合并 PR #94 的源码：`cargo fmt --check` 通过，Calcit 0.13.77 确认
  `calcit.cirru` 无 canonical formatting 变化。
- 以下验证已通过：Actions 来源静态审计、三个 workflow YAML 解析、strict Caps
  依赖解析、Calcit check/quality/dynamic-method 门禁、两份静态检查文档、97 个
  Rust 测试、strict Clippy 以及 release dylib 构建。
