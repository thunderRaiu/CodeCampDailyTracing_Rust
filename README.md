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
| 2026-01-10 | 704. 二分查找 | [d260111s0704_binary_search.rs](./src/solutions/d260111s0704_binary_search.rs) | [二分法细节](./notes/algorithm_summary.md#二分法) | ✅ |
| 2026-01-12 | 027. 移除元素 | [s0027_remove_element.rs](./src/solutions/s0027_remove_element.rs) | [双指针技巧](./notes/algorithm_summary.md#双指针) | 📅 |