# 清理共享宏后的导入 / Clean up imports after shared macro migration

- 删除共享同步导出宏替代本地 wrapper 后不再使用的 `run_buffer_adapter` 转发。
- 保留 Paint 阻塞事件循环所需的 descriptor、callback 与 blocking adapter。

---

- Removed the now-unused `run_buffer_adapter` re-export after replacing the local synchronous wrapper with the shared macro.
- Kept the descriptors, callback helper, and blocking adapter required by Paint's blocking event loop.
