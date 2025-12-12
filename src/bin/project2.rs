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

impl<T> BinaryTree<T> {
    pub fn new() -> Self { BinaryTree { root: None } }
    pub fn insert_root(&mut self, data: T) { self.root = Some(Box::new(Node::new(data))); }
    pub fn set_left(&mut self, data: T) { if let Some(ref mut node) = self.root { node.left = Some(Box::new(Node::new(data))); } }
    pub fn set_right(&mut self, data: T) { if let Some(ref mut node) = self.root { node.right = Some(Box::new(Node::new(data))); } }
    

    pub fn set_left_tree(&mut self, tree: BinaryTree<T>) {
         if let Some(ref mut node) = self.root { node.left = tree.root; }
    }
    pub fn set_right_tree(&mut self, tree: BinaryTree<T>) {
         if let Some(ref mut node) = self.root { node.right = tree.root; }
    }


    // 计算叶子节点数量
    pub fn count_leaf_nodes(&self) -> usize {
        Self::count_leaf_recursive(&self.root)
    }

    fn count_leaf_recursive(node_opt: &Option<Box<Node<T>>>) -> usize {
        match node_opt {
            None => 0,
            Some(node) => {
                // 如果左右孩子都为空，则为叶子节点，返回 1
                if node.left.is_none() && node.right.is_none() {
                    return 1;
                }
                // 否则递归累加左右子树的叶子数
                Self::count_leaf_recursive(&node.left) + Self::count_leaf_recursive(&node.right)
            }
        }
    }
}

// ==========================================
// 主函数演示
// ==========================================
fn main() {
    println!("=== Problem 1: Count Leaf Nodes ===");

    /*
         构建树:
             1
            / \
           2   3
          / \
         4   5
         
         叶子节点应为: 4, 5, 3 (共3个)
    */
    
    let mut tree = BinaryTree::new();
    tree.insert_root(1);
    
    let mut node2 = BinaryTree::new(); node2.insert_root(2); node2.set_left(4); node2.set_right(5);
    let mut node3 = BinaryTree::new(); node3.insert_root(3);
    
    tree.set_left_tree(node2);
    tree.set_right_tree(node3);

    let count = tree.count_leaf_nodes();
    println!("Leaf Count Result: {}", count);
    
    println!("\n[Complexity Analysis]");
    println!("Time Complexity: O(n)");
    println!("Reason: The algorithm must visit every node exactly once to determine if it is a leaf.");
}