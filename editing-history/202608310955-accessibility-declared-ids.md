# 裁剪前校验可访问性 ID / Validate accessibility IDs before clipping

- 为每帧可访问性状态增加独立的 declared-ID 集合。
- 在计算裁剪可见性之前登记并校验 ID，完全隐藏的节点也不能与后续节点重复。
- 新增“隐藏节点先注册、可见节点复用 ID”回归测试。

- Added a per-frame declared-ID set to accessibility state.
- Registers and validates IDs before clipping, so fully hidden nodes cannot collide with later nodes.
- Added a regression test for a hidden node followed by a visible node using the same ID.
