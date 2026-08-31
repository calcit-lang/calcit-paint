# Cookbook example CI order / Cookbook example 的 CI 顺序

## Problem / 问题

- The discoverable `validate-scene` example calls the public native FFI API.
  CI initially checked examples before `./build.sh`, so Linux could not load
  `dylibs/libcalcit_paint.so`.
- 可发现的 `validate-scene` example 会调用公开 native FFI API。CI 初始在
  `./build.sh` 之前检查 example，Linux 因而无法加载 `dylibs/libcalcit_paint.so`。

## Resolution / 解决

- Keep formatting and pure Markdown checks before the Rust build, then run the
  API-attached example check immediately after `./build.sh`, before the
  cookbook and native smokes that use the same dylib.
- 保持格式与纯 Markdown 检查在 Rust build 之前；随后紧接 `./build.sh` 运行附着于
  API 的 example 检查，再运行使用同一 dylib 的 cookbook 与 native smoke。
