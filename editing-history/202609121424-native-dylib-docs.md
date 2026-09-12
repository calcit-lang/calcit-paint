# Document native dylib acquisition / 文档化原生 dylib 的获取方式

## English

- Document how consumers obtain the native `cdylib`: install the module with `caps add calcit-lang/calcit-paint@<tag>` and run `caps`, which copies modules containing `build.sh` into a cached realization directory, runs the script, requires output under `dylibs/`, and records a `.calcit-native.cirru` receipt keyed by module/commit/Calcit version.
- Record the from-source path (`./build.sh`), the runtime resolution through `calcit-paint.util/get-dylib-path`, and per-platform system libraries.
- Note that no prebuilt cross-platform artifact exists yet, referencing #105 and its design research.
- Fix a latent Windows naming mismatch surfaced during review: Rust omits the `lib` prefix for Windows cdylibs, so `build.sh` now renames `calcit_paint.dll` to `dylibs/libcalcit_paint.dll` to match `calcit-paint.util/get-dylib-path`.

Validation: `calcit docs format-md README.md --check` and `calcit docs check-md README.md --failures-only`.

## 中文

- 文档化消费者如何获取原生 `cdylib`：用 `caps add calcit-lang/calcit-paint@<tag>` 安装模块并运行 `caps`；`caps` 会把含 `build.sh` 的模块复制到缓存的 realization 目录、执行脚本、要求 `dylibs/` 下有产物，并写入按 module/commit/Calcit 版本索引的 `.calcit-native.cirru` receipt。
- 记录源码路径（`./build.sh`）、运行期解析（`calcit-paint.util/get-dylib-path`）与各平台系统库。
- 说明目前尚无跨平台预编译产物，并引用 #105 及其设计调研。
- 修复 review 中暴露的 Windows 命名不一致：Rust 的 Windows cdylib 省略 `lib` 前缀，`build.sh` 现在把 `calcit_paint.dll` 重命名为 `dylibs/libcalcit_paint.dll`，以匹配 `calcit-paint.util/get-dylib-path`。

验证：`calcit docs format-md README.md --check` 与 `calcit docs check-md README.md --failures-only`。

Related: #105, #45.
