# Skia paint styles and runnable Calcit demos

## 中文

- 将纯色、线性渐变、径向渐变统一为 `PaintSource`，并让常用 shape 的填充和描边复用同一套 paint 描述。
- 新增结构化 `:stroke`，支持 width、cap、join、miter-limit、dash 与 dash-offset，同时保留旧版颜色和线宽字段兼容。
- 新增 `blend` 图层容器及一组常用 Skia blend mode，并对渐变 stop、坐标、半径、dash 和未知 mode 提供明确校验错误。
- 使用完整样式描述作为 key，缓存 Skia gradient shader 与 dash path effect；缓存达到 256 项时清空，样式任一字段变化都会自然失效。
- 在默认 `calcit.cirru` 场景中加入可直接运行的线性渐变、径向渐变、虚线描边和 multiply 混合 demo，并同步补齐中英双语 README 与单元测试。

## English

- Unified solid colors, linear gradients, and radial gradients as `PaintSource`, shared by fills and strokes on the common shapes.
- Added structured `:stroke` options for width, cap, join, miter limit, dash intervals, and dash offset while preserving legacy color/width fields.
- Added a `blend` layer container with a curated set of Skia blend modes and explicit validation for stops, coordinates, radii, dash patterns, and unknown modes.
- Cached Skia gradient shaders and dash path effects by the complete style description; caches clear at 256 entries and naturally invalidate whenever any style field changes.
- Added runnable linear-gradient, radial-gradient, dashed-stroke, and multiply-blend demos to the default `calcit.cirru` scene, together with bilingual README documentation and unit tests.
