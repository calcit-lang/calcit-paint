# Accessibility semantic tree / 可访问性语义树

- Added explicit `:accessibility` metadata to interactive scene anchors. Metadata is strict: stable `:id`, supported `:role`, nonempty `:label`, optional string `:value`, and boolean state flags.
- Rebuild the AccessKit tree from the rendered scene after each redraw. The adapter is constructed before the window becomes visible, preserving the cross-platform lifecycle requirement introduced by the preceding window-lifecycle work.
- Route platform `Focus` and `Click` requests through the existing focus lifecycle and `PaintTarget` boundary as the nominal `PaintEvent :accessibility-action` event.
- Extended the runnable Calcit demo with accessible button and text-input nodes; documented the protocol in English and Chinese.
- Verified with pinned Calcit checks/tests/quality/docs, Rust tests (86), strict clippy, release build, and one-frame native smoke.

## Knowledge / 知识点

`accesskit_winit::Adapter::with_event_loop_proxy` must run while the window is still invisible. Call `process_event` before application handling for every winit window event, and use `update_if_active` with a full `TreeUpdate` when handling `InitialTreeRequested` or synchronizing a fresh rendered scene. Keep semantic IDs independent from render paths; deterministic node IDs make tree updates stable across redraws.
