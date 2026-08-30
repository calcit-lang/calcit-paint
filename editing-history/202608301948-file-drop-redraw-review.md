# File drop redraw review / 文件拖放重绘 review

## 中文

- 根据 PR #66 review，文件 hover/drop/cancel 在成功 dispatch 后统一请求窗口重绘，与鼠标移动、滚轮等原生输入事件保持一致。
- callback 仍可通过 `request-frame!` 使用按需帧调度；无条件 redraw 同时保证兼容 API 用户仅更新 drawing data 时不会等待其他窗口事件。
- 非 UTF-8 hover/drop 路径仍只输出明确错误，不会为未送达 callback 的事件触发无意义重绘。

## English

- Address PR #66 review by requesting a window redraw after successfully dispatching file hover/drop/cancel, matching existing native input events such as pointer movement and wheel input.
- Callbacks may still use `request-frame!` for on-demand scheduling; the unconditional redraw also ensures compatible-API callers that only update drawing data do not wait for another window event.
- Invalid non-UTF-8 hover/drop paths still emit an explicit error without triggering a redraw for an event that was not delivered.
