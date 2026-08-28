# 将 kwd 内部术语更新为 tag

- Calcit 与 Cirru EDN 已统一使用 `tag` 描述 `:name` 类型，移除 Paint Rust
  代码中遗留的 `kwd` 与 `load_kwd` helper 命名。
- 将事件构造、shape 字段读取和相关测试统一收敛到 `extracter::tag`，并删除
  `primes` 中仅做转发的重复 helper。
- 本次只调整 crate 内部术语；生成与匹配的数据仍是 `Edn::Tag`，公开 FFI、
  Calcit shape 数据和事件字段保持不变。
