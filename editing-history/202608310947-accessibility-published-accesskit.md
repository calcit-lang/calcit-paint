# 使用已发布的 AccessKit 依赖 / Use published AccessKit dependencies

- 将 `accesskit_winit` 从尚未发布的 `0.34.0` 调整为 crates.io 可用的 `0.33.2`。
- 将直接依赖 `accesskit` 对齐到兼容版本 `0.24.1`，避免同一类型体系出现多个版本。
- 将语义树初始化适配为 AccessKit 0.24 的 `Tree` API，并通过依赖树与 strict clippy 验证。

- Changed `accesskit_winit` from the unpublished `0.34.0` to the crates.io release `0.33.2`.
- Aligned the direct `accesskit` dependency to the compatible `0.24.1` release to keep one shared type graph.
- Adapted semantic-tree initialization to AccessKit 0.24's `Tree` API and verified it with the dependency tree and strict clippy.
