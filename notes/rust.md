# 语法相关

## 二叉树实现

1. 显式的 `Option`: Rust 强制处理“空”的情况。不能简单地判断 `NULL`，必须用 `Option::None`，这从根本上杜绝了空指针异常。

2. `Rc<RefCell<T>>`:

    * 在 C++ 中，`TreeNode*` 既是引用也是所有者。

    * 在 Rust 中，`Rc (Reference Counted)` 负责多个地方同时指向同一个节点（比如父节点指向子节点，或者迭代器指向节点）。

    * `RefCell` 允许我们在运行时借用节点并修改它的 `left` 或 `right` 指针。

3. `borrow()` 和 `borrow_mut()`:

    * 只想读取值时，用 `borrow()`。

    * 当需要修改子节点指向时，用 `borrow_mut()`。
