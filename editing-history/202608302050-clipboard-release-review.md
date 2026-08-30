# Release clipboard outside the lock / 在锁外释放剪贴板

## 中文

- 根据 PR review 调整 `release()`：只在持锁期间从全局 slot 取走 `Clipboard` handle，随后在锁外显式销毁。
- 避免系统剪贴板析构中的潜在 OS 工作延长 mutex 临界区并阻塞并发访问。

## English

- Update `release()` after PR review: take the `Clipboard` handle from the global slot while locked, then explicitly drop it after the mutex guard is gone.
- Avoid extending the mutex critical section with possible OS work during clipboard teardown and blocking concurrent access.
