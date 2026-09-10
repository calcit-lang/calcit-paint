# Reject quoted Actions keys / 拒绝引号 Actions 键

## English

- Address the security review on PR #97 by recognizing YAML `uses` mapping keys
  written with single or double quotes.
- Quoted keys now enter the audit and fail its canonical-syntax check instead
  of being skipped, preventing a mutable action reference from bypassing the
  repository gate.
- Verify the fix with both double-quoted and single-quoted negative fixtures,
  plus the real repository workflows.

## 中文

- 处理 PR #97 的安全 review，识别使用单引号或双引号书写的 YAML `uses`
  mapping key。
- 引号 key 现在会进入审计并因非 canonical 语法而失败，不再被直接跳过，
  防止可移动 action 引用绕过仓库门禁。
- 使用双引号、单引号负向 fixture 与真实仓库 workflow 验证修复。
