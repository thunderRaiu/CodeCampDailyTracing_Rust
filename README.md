# CodeCampDailyTracing_Rust
------
Can I commit to checking in daily coding using Rust?
------

## Why Rust?

**Memory Safety and Ownership Mechanism**: 
When solving algorithm problems in Rust, I’m forced to think carefully about memory layout and lifetimes. These low-level constraints often expose issues early and help me write more robust code.

**Performance**: 
As a systems-level language, Rust delivers performance comparable to C++, which makes it well-suited for algorithm practice without worrying about efficiency bottlenecks.

**Toolchain**: 
Cargo provides a clean and powerful workflow for dependency management and testing. It makes the transition from isolated algorithm solutions to small experiments or projects very smooth.

**Mindset shift**: Moving from a mostly imperative style to Rust’s hybrid paradigm (ownership, traits, iterators, and functional patterns) has been a meaningful shift in how I think about programming.

## 文件结构与注释
```
CodeCampDailyTracing_Rust/
├── src/
│   ├── lib.rs          # 库文件
│   ├── main.rs         # 默认的可执行文件 (cargo run)
│   ├── bin/            # 其他独立的可执行文件
│   │   ├── complexity_test.rs  # 复杂度测试
│   │   └── tools.rs            # 其他辅助工具
│   └── solutions/      # 算法题解库
└── notes/
     ├── algrithm_summary.md  # 算法笔记
     └── rust.md              # rust语法笔记

```

## 📈 每日打卡记录

| 日期 | 题目 | 代码实现 | 学习笔记 | 状态 |
| :--- | :--- | :--- | :--- | :--- |
| 2026-01-10 | 704. 二分查找 | [d260111s0704_binary_search.rs](./src/solutions/d260111s0704_binary_search.rs) | [二分法细节](./notes/algorithm_summary.md#二分法) | ✅ |
| 2026-01-13 | 027. 移除元素 | [d260113s0027_remove_element.rs](./src/solutions/d260113s0027_remove_element.rs) | [双指针技巧](./notes/algorithm_summary.md#双指针) | ✅ |
| 2026-01-13 | 977. 有序数组的平方 | [d260113s0977_sorted_squres.rs](./src/solutions/d260113s0997_sorted_squres.rs) | [双指针技巧](./notes/algorithm_summary.md#双指针) | ✅ |
| 2026-01-14 | 209. 长度最小的子数组 | [d260114s0209_min_sub_array_len.rs](./src/solutions/d260114s0209_min_sub_array_len.rs) | [滑动窗口](./notes/algorithm_summary.md#滑动窗口) | ✅ |
| 2026-01-15 | 59. 螺旋矩阵 II | [d260115s0059_generate_matrix.rs](./src/solutions/d260115s0059_generate_matrix.rs) | [循环不变量原则](./notes/algorithm_summary.md#边界条件循环不变量原则) | ✅ |
| 2026-01-15 | 58. 区间和 |[d260115s1058_range_sum.rs](./src/solutions/d260115s1058_range_sum.rs) | [前缀和](./notes/algorithm_summary.md#前缀和) | ✅ |
| 2026-01-15 | 44. 开发商购买土地 |[d260115s1044_perchaseland.rs](./src/solutions/d260115s1044_perchaseland.rs) | [前缀和的平面化](./notes/algorithm_summary.md#前缀和的平面化) | ✅ |