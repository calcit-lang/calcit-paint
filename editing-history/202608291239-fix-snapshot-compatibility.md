# Preserve the Calcit 0.13.58 Snapshot format

## 中文

- PR #41 的首次 Actions 使用 Calcit 0.13.58 失败，错误为 Snapshot 的 `:files` key 不是 string/tag。
- 根因是本机全局 Calcit 0.13.60 在 `edit def` 后把受影响的 Snapshot file/definition key 序列化为 quoted symbol；0.13.58 无法读取该表示。
- 在临时 worktree 中构建并使用项目锁定的 Calcit 0.13.58，通过其 `edit` 与 `cursor` 命令从干净的 main Snapshot 重新生成 text-layout 变更；随后逐字节确认当前 Snapshot 与该生成结果一致，并以 0.13.58 运行 check 与 README check。
- 已按 Agent 指南提交 Calcit 核心 Issue #515（中英双语），记录 0.13.60 编辑器与 0.13.58 消费端的格式兼容性缺陷。

## English

- The first Actions run for PR #41 failed under Calcit 0.13.58 because a `:files` key in the Snapshot was no longer a string/tag.
- The global local Calcit 0.13.60 serialized affected Snapshot file/definition keys as quoted symbols after `edit def`; 0.13.58 cannot read that representation.
- Built and used the project-pinned Calcit 0.13.58 in a temporary worktree to regenerate the text-layout changes from a clean main Snapshot using its `edit` and `cursor` commands. The current Snapshot was then byte-compared with that output and checked with Calcit 0.13.58, including README validation.
- Filed bilingual Calcit core Issue #515 under the Agent protocol to record the 0.13.60 editor / 0.13.58 consumer format-compatibility defect.
