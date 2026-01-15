# 语法相关

## 滑动窗口实现

* `i32::MAX`: 这种常量通常通过类型名直接调用。

* 三元运算符的替代: Rust 没有 `condition ? a : b`。

    简单情况直接用 `if result == i32::MAX { 0 } else { result }`。

    因为 Rust 是基于表达式的，整个 `if `块会返回一个值。

* `std::cmp::min`简化代码:  `result = std::cmp::min(result, (right - left + 1) as i32)`

## 循环不变量

* 反向遍历 rev(): 在 Rust 中，为了安全，通常使用 (start..end).rev()。
    注意 `rev() `作用于 `Range` 时，依然遵循左闭右开原则。

* 二维数组初始化: `vec![vec![0; n]; n]` 是 Rust 初始化 n x n 矩阵的标准写法。

* 循环不变量 (Loop Invariant): 最容易写错的地方是 usize 的减法。通过坚持 “左闭右开”确保了每个 `for` 循环处理的长度是一致的，逻辑上极其对称。

## 二叉树实现

1. 显式的 `Option`: Rust 强制处理“空”的情况。不能简单地判断 `NULL`，必须用 `Option::None`，这从根本上杜绝了空指针异常。

2. `Rc<RefCell<T>>`:

    * 在 C++ 中，`TreeNode*` 既是引用也是所有者。

    * 在 Rust 中，`Rc (Reference Counted)` 负责多个地方同时指向同一个节点（比如父节点指向子节点，或者迭代器指向节点）。

    * `RefCell` 允许我们在运行时借用节点并修改它的 `left` 或 `right` 指针。

3. `borrow()` 和 `borrow_mut()`:

    * 只想读取值时，用 `borrow()`。

    * 当需要修改子节点指向时，用 `borrow_mut()`。


##