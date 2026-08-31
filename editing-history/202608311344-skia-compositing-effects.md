# Skia compositing effects / Skia 合成效果

- 新增三个严格的 scene group：`drop-shadow`（有限偏移、非负 X/Y sigma 与 HSL(A) 颜色）、`blur`（内容模糊）和 `color-filter`（恰好 20 个有限 RGBA 行主序矩阵系数）。三者通过 `SaveLayer` 与 Skia image filter 合成，不改变子节点的指针、键盘、焦点或无障碍语义。
- 缓存子树复用同一离屏绘制路径；效果受 `cached-group` 声明的本地 raster surface 边界裁切，因此调用方要为 blur/shadow 扩散留出边距。`render-to-png` 同样使用这一路径。
- 默认可运行 Calcit demo 直接展示阴影、内容模糊和灰度色彩矩阵；README 同步中英双语 API、边界与示例。新增离屏像素、严格诊断与 cached-group 一致性测试，并为共享 subtree cache 测试加串行锁，避免并发测试互相清空全局 cache。

## Knowledge / 知识点

`skia-safe 0.97` 的 image filter 通过 `image_filters::{drop_shadow, blur, color_filter}` 构造，并写入 layer paint 的 `set_image_filter`。`drop_shadow` 接受 `Color4f` 与可选 color space；对当前 CPU sRGB surface 传 `Color4f::from(Color)` 和 `None` 即可。颜色矩阵应交由 `color_filters::matrix_row_major`，再包入 `image_filters::color_filter`，以保持与其他图像过滤器相同的 layer 生命周期。

## English summary

- Add three strict scene groups: `drop-shadow` (finite offsets, non-negative X/Y sigma, and HSL(A) color), content-only `blur`, and `color-filter` (exactly 20 finite row-major RGBA coefficients). Each is composited through `SaveLayer` and a Skia image filter without changing descendant pointer, keyboard, focus, or accessibility semantics.
- Cached subtrees reuse the same offscreen drawing path. Effects are clipped by the declared local `cached-group` raster surface, so callers must reserve padding for blur/shadow spread. `render-to-png` uses this same path.
- The runnable Calcit demo renders shadow, content blur, and a grayscale matrix. README documents the bilingual API, limits, and examples. New coverage checks offscreen pixels, strict diagnostics, and cached-group consistency; serialized cache tests prevent parallel tests from clearing the shared global cache underneath one another.
