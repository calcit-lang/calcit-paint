# Accessibility-ready window lifecycle / 为可访问性准备的窗口生命周期

- Deferred creation of the native window, OpenGL context, and Skia surface until `ApplicationHandler::resumed`, where winit provides `ActiveEventLoop`.
- The window now starts invisible and becomes visible only after the native drawing environment is ready. This preserves a compliant installation point for platform accessibility adapters before first display.
- Kept the existing single-window lifecycle, initial `:ready` dispatch, frame scheduling, theme observation, render error handling, and one-frame smoke behavior intact. `Env` owns framebuffer metadata so resize continues to recreate the correct Skia surface after initialization.
