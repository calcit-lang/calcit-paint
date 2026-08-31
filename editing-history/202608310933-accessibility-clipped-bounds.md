# Accessibility clipping review fix / 可访问性裁剪 review 修复

- Propagate the renderer's active clip stack into accessibility registration.
- Intersect semantic bounds with transformed clip bounds; omit a node when no visible rectangle remains or its exposed rectangle is outside a rounded/transformed clip.
- Add a regression test for both partially visible and fully hidden semantic nodes.
- Verified the review's dependency concern against current Cargo metadata: `accesskit_winit 0.34.0` is published and resolves with `accesskit 0.25.0`; no downgrade is required.

## Knowledge / 知识点

Accessibility bounds must never include a visually clipped interaction region. In a tree API that represents bounds as an axis-aligned rectangle, prefer omitting an ambiguous partially visible node over publishing a rectangle whose corners escape the active clip stack.
