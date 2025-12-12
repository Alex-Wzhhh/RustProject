use std::collections::VecDeque;
use std::fmt::Debug;

// ==========================================
// 二叉树节点定义 (Node Definition)
// ==========================================
#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, left: None, right: None }
    }
}

// ==========================================
// 二叉树类定义 (BinaryTree Class)
// ==========================================
#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

// 1. 析构函数 (Destructor - Drop Trait)
impl<T> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        // 使用 take() 拿出 root，确保 self.root 变为 None，避免重复释放
        if let Some(root) = self.root.take() {
            stack.push(root);
        }
        while let Some(mut node) = stack.pop() {
            if let Some(left) = node.left.take() { stack.push(left); }
            if let Some(right) = node.right.take() { stack.push(right); }
        }
    }
}

// 2. 拷贝构造函数 (Copy Constructor - Clone Trait)
impl<T: Clone> Clone for BinaryTree<T> {
    fn clone(&self) -> Self {
        BinaryTree { root: self.root.clone() }
    }
}

impl<T: Clone + Debug> BinaryTree<T> {
    // --- ADT 5.1 基础函数 ---
    pub fn new() -> Self { BinaryTree { root: None } }
    
    pub fn is_empty(&self) -> bool { self.root.is_none() }


    // 这里的 mut left/right 允许修改传入的树结构
    pub fn from_subtrees(mut left: BinaryTree<T>, item: T, mut right: BinaryTree<T>) -> Self {
        let mut node = Node::new(item);
        // take() 将值取出，留下 None。这样当 left/right 被 Drop 时是安全的。
        node.left = left.root.take(); 
        node.right = right.root.take();
        BinaryTree { root: Some(Box::new(node)) }
    }

    pub fn left_subtree(&self) -> BinaryTree<T> {
        match &self.root {
            Some(node) => BinaryTree { root: node.left.clone() },
            None => BinaryTree::new(),
        }
    }

    pub fn right_subtree(&self) -> BinaryTree<T> {
        match &self.root {
            Some(node) => BinaryTree { root: node.right.clone() },
            None => BinaryTree::new(),
        }
    }

    pub fn root_data(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.data)
    }

    // 辅助构建函数 
    pub fn insert_root(&mut self, data: T) { self.root = Some(Box::new(Node::new(data))); }
    pub fn set_left(&mut self, data: T) { if let Some(ref mut node) = self.root { node.left = Some(Box::new(Node::new(data))); } }
    pub fn set_right(&mut self, data: T) { if let Some(ref mut node) = self.root { node.right = Some(Box::new(Node::new(data))); } }

    // 获取迭代器的接口
    pub fn iter_preorder(&self) -> PreOrderIter<T> { PreOrderIter::new(self.root.as_deref()) }
    pub fn iter_inorder(&self) -> InOrderIter<T> { InOrderIter::new(self.root.as_deref()) }
    pub fn iter_postorder(&self) -> PostOrderIter<T> { PostOrderIter::new(self.root.as_deref()) }
    pub fn iter_levelorder(&self) -> LevelOrderIter<T> { LevelOrderIter::new(self.root.as_deref()) }
}


// 1. 前序迭代器
pub struct PreOrderIter<'a, T> { stack: Vec<&'a Node<T>> }
impl<'a, T> PreOrderIter<'a, T> {
    fn new(root: Option<&'a Node<T>>) -> Self {
        let mut stack = Vec::new();
        if let Some(node) = root { stack.push(node); }
        PreOrderIter { stack }
    }
}
impl<'a, T> Iterator for PreOrderIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Some(ref right) = node.right { self.stack.push(right); }
        if let Some(ref left) = node.left { self.stack.push(left); }
        Some(&node.data)
    }
}

// 2. 中序迭代器
pub struct InOrderIter<'a, T> { stack: Vec<&'a Node<T>>, current: Option<&'a Node<T>> }
impl<'a, T> InOrderIter<'a, T> {
    fn new(root: Option<&'a Node<T>>) -> Self { InOrderIter { stack: Vec::new(), current: root } }
}
impl<'a, T> Iterator for InOrderIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.left.as_deref();
        }
        if let Some(node) = self.stack.pop() {
            self.current = node.right.as_deref();
            return Some(&node.data);
        }
        None
    }
}

// 3. 后序迭代器
pub struct PostOrderIter<'a, T> { stack: Vec<&'a Node<T>>, last_visited: Option<&'a Node<T>> }

impl<'a, T> PostOrderIter<'a, T> {
    fn new(root: Option<&'a Node<T>>) -> Self {
        let mut stack = Vec::new();
        if let Some(r) = root { stack.push(r); }
        PostOrderIter { stack, last_visited: None }
    }
    fn is_visited(&self, node: &Node<T>) -> bool {
        if let Some(last) = self.last_visited { std::ptr::eq(node, last) } else { false }
    }
    fn is_visited_opt(&self, node: Option<&Node<T>>) -> bool {
        if let (Some(n), Some(last)) = (node, self.last_visited) { std::ptr::eq(n, last) } else { false }
    }
}

impl<'a, T> Iterator for PostOrderIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let peek_node = *self.stack.last()?;
            
            // 检查左孩子是否需要入栈
            if let Some(left) = &peek_node.left {
                // self.is_visited 被调用
                if !self.is_visited(left) && !self.is_visited_opt(peek_node.right.as_deref()) {
                     self.stack.push(left); continue;
                }
            }
            // 检查右孩子是否需要入栈
            if let Some(right) = &peek_node.right {
                 if !self.is_visited(right) {
                     self.stack.push(right); continue;
                 }
            }
            // 访问当前节点
            let node = self.stack.pop()?;
            self.last_visited = Some(node);
            return Some(&node.data);
        }
    }
}

// 4. 层序迭代器
pub struct LevelOrderIter<'a, T> { queue: VecDeque<&'a Node<T>> }
impl<'a, T> LevelOrderIter<'a, T> {
    fn new(root: Option<&'a Node<T>>) -> Self {
        let mut queue = VecDeque::new();
        if let Some(node) = root { queue.push_back(node); }
        LevelOrderIter { queue }
    }
}
impl<'a, T> Iterator for LevelOrderIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;
        if let Some(ref left) = node.left { self.queue.push_back(left); }
        if let Some(ref right) = node.right { self.queue.push_back(right); }
        Some(&node.data)
    }
}

// ==========================================
// 主函数演示
// ==========================================
fn main() {
    println!("=== Project 10: Binary Tree ADT & Iterators ===");
    // 构建树: 1 -> (2 -> (4, 5), 3)
    let mut left = BinaryTree::new(); left.insert_root(2); left.set_left(4); left.set_right(5);
    let mut right = BinaryTree::new(); right.insert_root(3);
    
    // from_subtrees 会消耗掉 left 和 right 的所有权
    let tree = BinaryTree::from_subtrees(left, 1, right);

    print!("Pre-order:   "); for v in tree.iter_preorder() { print!("{} ", v); } println!();
    print!("In-order:    "); for v in tree.iter_inorder() { print!("{} ", v); } println!();
    print!("Post-order:  "); for v in tree.iter_postorder() { print!("{} ", v); } println!();
    print!("Level-order: "); for v in tree.iter_levelorder() { print!("{} ", v); } println!();
}