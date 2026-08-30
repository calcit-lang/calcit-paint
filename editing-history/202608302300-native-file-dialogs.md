# Native file dialogs / 原生文件对话框

- Added an asynchronous single-window request queue for native open/save dialogs. A worker owns the blocking platform dialog and sends its terminal result back through a winit user event, leaving Calcit callbacks and the main event loop responsive.
- Defined strict nominal Calcit options, filter, and result payloads. Results distinguish selected, cancelled, and failed states, preserve `FsPath`, reject malformed options, and serialize concurrent dialog ownership.
- Extended the runnable Calcit demo: `T` opens a filtered image chooser and `S` opens a PNG save chooser; the existing visible status line reports terminal results.
- Added Rust transport/parser tests, a typed Calcit decoder test, bilingual README documentation, and an architecture plan under `docs/architectures/`.
