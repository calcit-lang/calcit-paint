# crates.io metadata / crates.io 元数据

## 中文

- 为 `calcit_paint` 补齐 crates.io 强制要求的描述与 MIT 许可证元数据，并声明上游仓库地址。
- `cargo publish` 会在上传前拒绝缺少 `description`、`license` 的新版本；应在创建 release/tag 前用 `cargo package` 验证。

## English

- Added the description and MIT license metadata required by crates.io, together with the upstream repository URL.
- `cargo publish` rejects new versions missing `description` or `license` before upload; verify with `cargo package` before creating a release/tag.
