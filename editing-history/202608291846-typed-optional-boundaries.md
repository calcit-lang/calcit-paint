# Typed optional boundaries / 类型化可选边界

## 中文

- 将 touch-area/key-listener 的 `:action`、`:path`、`:data` 收拢为 Rust
  `EventTarget`，内部用 `Option<Edn>` 表达缺失；省略字段和显式 `nil` 均兼容。
- 事件发出边界继续补齐三个历史字段，缺失值仍编码为 `nil`，避免破坏现有回调。
- Calcit 副作用 wrapper 改为显式 `Unit`，绘制 payload 使用泛型，文字测量结果收窄为
  `Map<Tag, Number>`；可运行 demo 删除六个无意义的 `nil` 占位。
- blocking callback 的返回值必须是可序列化 Cirru EDN；公开 callback 返回类型以泛型
  `R` 保持兼容，内部 adapter 丢弃其返回值并向 ABI 返回 `:handled`。callback 输入仍保留
  一个 Dynamic，因为协议先发送兼容用 `nil`，再发送异构 event map。文字选项 map 的
  异构 value 是第二个保留的 Dynamic。
- 新增 reviewed quality baseline 和 dynamic-method CI gate，阻止弱类型债务回升。

## English

- Consolidated touch-area/key-listener `:action`, `:path`, and `:data` into a Rust
  `EventTarget` with `Option<Edn>` fields; omitted fields and explicit `nil` remain compatible.
- Event emission still supplies the three legacy keys and serializes absent values as `nil`.
- Calcit side-effect wrappers now return explicit `Unit`; drawing payloads are generic and text
  measurement returns `Map<Tag, Number>`. The runnable demo drops six meaningless nil placeholders.
- Blocking callback results must be Cirru-EDN serializable. The public callback result stays
  generic as `R`; an internal adapter discards it and returns `:handled` to the ABI. One Dynamic
  remains for the legacy initial `nil` plus heterogeneous event maps, and one remains for
  heterogeneous text-option map values.
- Added a reviewed quality baseline and a zero-dynamic-method CI gate.

## Verification / 验证

- `cargo fmt --check`
- `cargo test` (26 passed)
- `cargo clippy --all-targets -- -D warnings`
- `caps --strict --ci`
- Calcit 0.13.58 `--check-only`, weak-type audit, quality baseline gate, and dynamic-method gate
- `./build.sh`, exported-symbol audit, and real macOS `CALCIT_PAINT_SMOKE_ONCE=1` smoke
