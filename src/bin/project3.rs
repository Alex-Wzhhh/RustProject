use std::fmt::Debug;

// 为了独立运行，包含基础的树结构定义
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self { Node { data, left: None, right: None } }
}

pub struct BinaryTree<T> { root: Option<Box<Node<T>>> }

impl<T: Debug> BinaryTree<T> {
    pub fn new() -> Self { BinaryTree { root: None } }
    pub fn insert_root(&mut self, data: T) { self.root = Some(Box::new(Node::new(data))); }
    pub fn set_left(&mut self, data: T) { if let Some(ref mut node) = self.root { node.left = Some(Box::new(Node::new(data))); } }
    pub fn set_right(&mut self, data: T) { if let Some(ref mut node) = self.root { node.right = Some(Box::new(Node::new(data))); } }
    pub fn set_left_tree(&mut self, tree: BinaryTree<T>) { if let Some(ref mut node) = self.root { node.left = tree.root; } }
    pub fn set_right_tree(&mut self, tree: BinaryTree<T>) { if let Some(ref mut node) = self.root { node.right = tree.root; } }

    // 简单的层序打印，用于验证交换结果
    pub fn print_level_order(&self) {
        let mut queue = std::collections::VecDeque::new();
        if let Some(node) = &self.root { queue.push_back(node); }
        
        while let Some(node) = queue.pop_front() {
            print!("{:?} ", node.data);
            if let Some(left) = &node.left { queue.push_back(left); }
            if let Some(right) = &node.right { queue.push_back(right); }
        }
        println!();
    }

    // ==========================================
    // 核心解答: 交换左右子树
    // ==========================================
    pub fn swap_tree(&mut self) {
        Self::swap_recursive(&mut self.root);
    }

    fn swap_recursive(node_opt: &mut Option<Box<Node<T>>>) {
        if let Some(node) = node_opt {
            // 1. 物理交换当前节点的左右指针 (Memory Swap)
            std::mem::swap(&mut node.left, &mut node.right);
            
            // 2. 递归处理子节点
            Self::swap_recursive(&mut node.left);
            Self::swap_recursive(&mut node.right);
        }
    }
}

// ==========================================
// 主函数演示
// ==========================================
fn main() {
    println!("=== Problem 2: Swap Tree ===");

    /*
         原始树:
             1
            / \
           2   3
          / \
         4   5
    */
    let mut tree = BinaryTree::new();
    tree.insert_root(1);
    let mut node2 = BinaryTree::new(); node2.insert_root(2); node2.set_left(4); node2.set_right(5);
    let mut node3 = BinaryTree::new(); node3.insert_root(3);
    tree.set_left_tree(node2);
    tree.set_right_tree(node3);

    print!("Original (Level-order): ");
    tree.print_level_order();

    // 执行交换
    tree.swap_tree();

    /*
         交换后应为:
             1
            / \
           3   2
              / \
             5   4
    */
    print!("Swapped  (Level-order): ");
    tree.print_level_order();
}