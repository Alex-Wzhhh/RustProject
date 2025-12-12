use std::rc::Rc;
use std::cell::{RefCell, Ref}; 



type Link = Rc<RefCell<MatrixNode>>;

struct MatrixNode {
    row: usize,
    col: usize,
    value: i32,
    right: Option<Link>,
    down: Option<Link>,
}

impl MatrixNode {
    fn new(row: usize, col: usize, value: i32) -> Self {
        MatrixNode {
            row,
            col,
            value,
            right: None,
            down: None,
        }
    }
}


pub struct SparseMatrix {
    head: Link, 
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let head_node = Rc::new(RefCell::new(MatrixNode::new(rows, cols, 0)));
        
        head_node.borrow_mut().right = Some(head_node.clone());
        head_node.borrow_mut().down = Some(head_node.clone());

        SparseMatrix { head: head_node }
    }

    fn dims(&self) -> (usize, usize) {
        let h = self.head.borrow();
        (h.row, h.col)
    }

    pub fn from_triplets(rows: usize, cols: usize, mut triplets: Vec<(usize, usize, i32)>) -> Self {
        let matrix = SparseMatrix::new(rows, cols);
        let head = matrix.head.clone();
        if triplets.is_empty() { return matrix; }
        triplets.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let nodes: Vec<Link> = triplets.iter()
            .map(|&(r, c, v)| Rc::new(RefCell::new(MatrixNode::new(r, c, v))))
            .collect();
        let mut last = head.clone();
        for node in &nodes {
            last.borrow_mut().right = Some(node.clone());
            last = node.clone();
        }
        last.borrow_mut().right = Some(head.clone()); 
        let mut node_indices: Vec<usize> = (0..nodes.len()).collect();
        node_indices.sort_by(|&i, &j| {
            let n_i = nodes[i].borrow();
            let n_j = nodes[j].borrow();
            n_i.col.cmp(&n_j.col).then(n_i.row.cmp(&n_j.row))
        });
        let mut last = head.clone();
        for idx in node_indices {
            let node = &nodes[idx];
            last.borrow_mut().down = Some(node.clone());
            last = node.clone();
        }
        last.borrow_mut().down = Some(head.clone());
        matrix
    }
    //加法
    pub fn add(&self, other: &SparseMatrix) -> SparseMatrix {
        let (rows, cols) = self.dims();
        let result = SparseMatrix::new(rows, cols);
        let res_head = result.head.clone();

        let mut p_a = self.head.borrow().right.clone().unwrap();
        let mut p_b = other.head.borrow().right.clone().unwrap();

        let mut last_right = res_head.clone(); 
        
        let mut col_heads: Vec<Option<Link>> = vec![None; cols + 1];
        let mut col_tails: Vec<Option<Link>> = vec![None; cols + 1];

        loop {
            let a_is_head = Rc::ptr_eq(&p_a, &self.head);
            let b_is_head = Rc::ptr_eq(&p_b, &other.head);

            if a_is_head && b_is_head { break; }

            let next_a;
            let next_b;

            let mut val;
            let mut curr_row;
            let mut curr_col;
            let mut has_node = false;

            let key_a = if a_is_head { (usize::MAX, usize::MAX) } else { (p_a.borrow().row, p_a.borrow().col) };
            let key_b = if b_is_head { (usize::MAX, usize::MAX) } else { (p_b.borrow().row, p_b.borrow().col) };

            if key_a < key_b {
                { 
                    let node_a: Ref<MatrixNode> = p_a.borrow(); 
                    curr_row = key_a.0;
                    curr_col = key_a.1;
                    val = node_a.value;
                    next_a = node_a.right.clone().unwrap();
                }
                p_a = next_a; 
                has_node = true;
            } else if key_b < key_a {

                { 
                    let node_b: Ref<MatrixNode> = p_b.borrow(); 
                    curr_row = key_b.0;
                    curr_col = key_b.1;
                    val = node_b.value;
                    next_b = node_b.right.clone().unwrap();
                } 
                p_b = next_b; 
                has_node = true;
            } else {
                { 
                    let node_a = p_a.borrow(); 
                    let node_b = p_b.borrow(); 
                    curr_row = key_a.0;
                    curr_col = key_a.1;
                    val = node_a.value + node_b.value;
                    next_a = node_a.right.clone().unwrap();
                    next_b = node_b.right.clone().unwrap();
                } 
                
                p_a = next_a;
                p_b = next_b;
                
                if val != 0 { has_node = true; }
            }

            if has_node {
                let new_node = Rc::new(RefCell::new(MatrixNode::new(curr_row, curr_col, val)));
                last_right.borrow_mut().right = Some(new_node.clone());
                last_right = new_node.clone();
                if col_heads[curr_col].is_none() {
                    col_heads[curr_col] = Some(new_node.clone());
                } else {
                    let tail = col_tails[curr_col].as_ref().unwrap();
                    tail.borrow_mut().down = Some(new_node.clone());
                }
                col_tails[curr_col] = Some(new_node.clone());
            }
        }
        last_right.borrow_mut().right = Some(res_head.clone());

        let mut last_down = res_head.clone();
        
        for j in 1..=cols {
            if let Some(col_first) = &col_heads[j] {
                last_down.borrow_mut().down = Some(col_first.clone());
                last_down = col_tails[j].as_ref().unwrap().clone();
            }
        }
        last_down.borrow_mut().down = Some(res_head.clone());

        result
    }
    //乘法
    pub fn multiply(&self, other: &SparseMatrix) -> SparseMatrix {
        let (r_a, c_a) = self.dims();
        let (r_b, c_b) = other.dims();
        
        if c_a != r_b {
            panic!("Dimensions mismatch for multiplication");
        }

        let mut triplets: Vec<(usize, usize, i32)> = Vec::new();

        let mut p_a = self.head.borrow().right.clone().unwrap();
        
        let mut current_row_a_idx = 0;
        let mut row_a_nodes: Vec<Link> = Vec::new();

        loop {
            let a_is_head = Rc::ptr_eq(&p_a, &self.head);
            
            let next_a;

            let row_changed = !a_is_head && p_a.borrow().row != current_row_a_idx;
            
            if row_changed || a_is_head {
                if !row_a_nodes.is_empty() {
                    //当前行 A[i] 与 整个矩阵 B 的乘法
                    let mut p_b = other.head.borrow().down.clone().unwrap();
                    let mut current_col_b_idx = 0;
                    let mut col_b_sum = 0;
                    
                    loop {
                        let b_is_head = Rc::ptr_eq(&p_b, &other.head);
                        let next_b;

                        let b_row_changed = !b_is_head && p_b.borrow().col != current_col_b_idx;

                        if b_row_changed || b_is_head {
                            if col_b_sum != 0 {
                                triplets.push((current_row_a_idx, current_col_b_idx, col_b_sum));
                                col_b_sum = 0;
                            }
                            if b_is_head { break; }
                            current_col_b_idx = p_b.borrow().col;
                        }
                        {
                            let node_b = p_b.borrow(); 
                            let b_row = node_b.row;
                            let b_val = node_b.value;
                            next_b = node_b.down.clone().unwrap();

                            for node_a in &row_a_nodes {
                                let n_a = node_a.borrow();
                                if n_a.col == b_row {
                                    col_b_sum += n_a.value * b_val;
                                }
                            }
                        } 

                        p_b = next_b;
                    }
                }
                
                row_a_nodes.clear();
                if a_is_head { break; }
                {
                    current_row_a_idx = p_a.borrow().row;
                }
                
            }
            {
                let node_a = p_a.borrow(); 
                row_a_nodes.push(p_a.clone());
                next_a = node_a.right.clone().unwrap();
            }
            p_a = next_a; 
        }

        SparseMatrix::from_triplets(r_a, c_b, triplets)
    }

    pub fn print(&self) {
        let (rows, cols) = self.dims();
        println!("Matrix ({}x{}):", rows, cols);
        

        let mut p = self.head.borrow().right.clone().unwrap();
        

        for r in 1..=rows {
            for c in 1..=cols {
                let mut val = 0;
                let is_head = Rc::ptr_eq(&p, &self.head);
                let mut matched = false;

                if !is_head {
                    let node = p.borrow();
                    if node.row == r && node.col == c {
                        val = node.value;
                        matched = true;
                    }
                }

                if matched {
                    let next = p.borrow().right.clone().unwrap();
                    p = next;
                }
                
                print!("{:4} ", val);
            }

            println!();
        }
        println!("----------------------");
    }
}

fn main() {

    let a_triplets = vec![
        (1, 1, 2), 
        (2, 1, 4), (2, 4, 3),
        (4, 1, 8), (4, 4, 1),
        (5, 3, 6)
    ];

    let a = SparseMatrix::from_triplets(5, 4, a_triplets);
    println!("Matrix A:");
    a.print();

    let b_triplets = vec![
        (1, 1, 1), (1, 3, 2),
        (2, 2, 5),
        (4, 1, 3), (4, 3, 1)
    ];
    let b = SparseMatrix::from_triplets(4, 3, b_triplets);
    println!("Matrix B:");
    b.print();


    println!("(f) A + A:");
    let sum = a.add(&a);
    sum.print();


    println!("(h) A * B:");
    let product = a.multiply(&b);
    product.print();
}