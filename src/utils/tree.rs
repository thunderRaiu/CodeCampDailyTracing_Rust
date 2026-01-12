use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

// 定义二叉树节点
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 根据数组构造二叉树 (层序构造)
pub fn construct_binary_tree(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() || vec[0] == -1 {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vec[0])));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < vec.len() {
        if let Some(parent) = queue.pop_front() {
            // 处理左子节点
            if i < vec.len() && vec[i] != -1 {
                let left_node = Rc::new(RefCell::new(TreeNode::new(vec[i])));
                parent.borrow_mut().left = Some(Rc::clone(&left_node));
                queue.push_back(left_node);
            }
            i += 1;

            // 处理右子节点
            if i < vec.len() && vec[i] != -1 {
                let right_node = Rc::new(RefCell::new(TreeNode::new(vec[i])));
                parent.borrow_mut().right = Some(Rc::clone(&right_node));
                queue.push_back(right_node);
            }
            i += 1;
        }
    }
    Some(root)
}

// 层序遍历打印二叉树
pub fn print_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = VecDeque::new();
    if let Some(r) = root {
        queue.push_back(Some(r));
    }

    while !queue.is_empty() {
        let size = queue.len();
        let mut level_vals = Vec::new();

        for _ in 0..size {
            if let Some(Some(node)) = queue.pop_front() {
                level_vals.push(node.borrow().val);
                
                // 将左右子节点加入队列，即便是 None 也要占位以便观察结构
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            } else {
                level_vals.push(-1); // 用 -1 表示 null
            }
        }

        // 如果整层都是 -1，说明到底了，不打印
        if level_vals.iter().all(|&x| x == -1) {
            break;
        }

        for val in level_vals {
            if val == -1 { print!("null "); }
            else { print!("{} ", val); }
        }
        println!();
    }
}