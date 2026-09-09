# Creative Art preview entry / Creative Art 预览入口

## English

- Add `calcit-paint.creative-art` as the default runnable Calcit entry for the
  0.1.0 Creative Art Preview milestone. Keep `calcit-paint.main` as the broader
  integration/API gallery.
- Build a deterministic scene from explicit seed/time inputs using existing
  public Paint capabilities: gradients, paths, transforms, clipping,
  compositing effects, typed events, accessibility metadata, on-demand frames,
  and offscreen PNG export.
- Add click/`R` regeneration, Space pause/resume, explicit `P` export, and `Q`
  close controls. Startup remains side-effect free with respect to files.
- Add a discoverable definition example and an executable CI smoke that first
  validates the scene, exports the same fixed input twice, compares PNG bytes,
  decodes RGBA pixels, and checks visual variety.
- Retain exactly one reviewed `Map<Tag, Dynamic>` return boundary on
  `build-art-scene`: Paint's recursive scene DSL is heterogeneous. Document the
  boundary and validate it before rendering instead of misclassifying it as a
  type slot or JS FFI.
- Update the bilingual README and cookbook to describe the 0.1.0 scope,
  controls, non-goals, default entry, and Agent-oriented verification path.

Validation used the project-pinned Calcit 0.13.77:

- strict Caps resolution, canonical Snapshot formatting, `--check-only`, nine
  Calcit tests, reviewed quality baseline, and zero dynamic-method findings
- discoverable examples plus README/cookbook `format-md` and `check-md`
- executable cookbook and deterministic 960x720 Creative Art PNG smokes
- real macOS blocking-window smoke with `CALCIT_PAINT_SMOKE_ONCE=1`
- `cargo fmt --check`, 96 Rust tests, strict Clippy, and a release dylib build

## 中文

- 新增 `calcit-paint.creative-art`，作为 0.1.0 Creative Art Preview milestone
  的默认可运行 Calcit 入口；`calcit-paint.main` 继续作为更完整的综合/API gallery。
- 使用显式 seed/time 与既有 Paint 公共能力构造确定性场景：渐变、路径、变换、裁剪、
  合成效果、强类型事件、无障碍元数据、按需帧以及离屏 PNG 导出。
- 增加点击/`R` 重新生成、空格暂停/继续、`P` 显式导出和 `Q` 关闭控制；启动过程不会
  自行写文件。
- 增加可发现的 definition example，以及可执行 CI smoke：先校验 scene，再以相同固定
  输入导出两次，比较 PNG 字节，并解码 RGBA 像素检查视觉丰富度。
- 在 `build-art-scene` 上只保留一个经审阅的 `Map<Tag, Dynamic>` 返回边界：Paint 的
  递归 scene DSL 本身是异构结构。该边界有明确文档并在绘制前经过校验，不会被误标为
  type slot 或 JS FFI。
- 更新双语 README 与 cookbook，说明 0.1.0 范围、控制方式、非目标、默认入口以及面向
  Agent 的验证路径。

验证使用项目锁定的 Calcit 0.13.77：

- strict Caps 解析、canonical Snapshot 格式、`--check-only`、九个 Calcit 测试、已审阅
  quality baseline 与动态 method 零命中
- 可发现 examples，以及 README/cookbook 的 `format-md` 与 `check-md`
- 可执行 cookbook 与确定性 960x720 Creative Art PNG smoke
- 使用 `CALCIT_PAINT_SMOKE_ONCE=1` 的真实 macOS blocking-window smoke
- `cargo fmt --check`、96 个 Rust 测试、strict Clippy 与 release dylib 构建
