use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeColor {
	Red,	
	Black, 
}

type Link<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug, Clone)]
pub struct TreeNode<T> where T: Clone {
	pub color: NodeColor,
	pub value: T,
    pub parent: Option<Weak<RefCell<TreeNode<T>>>>,
	left: Option<Link<T>>, 
	right: Option<Link<T>>
}

// Define the Red-Black Tree itself
#[derive(Debug)]
pub struct RedBlackTree<T> where T: Clone{
    root: Option<Link<T>>,
    rotation_function_inner: fn(&mut Self, &Link<T>),
    rotation_function_extern: fn(&mut Self, &Link<T>),
}

impl<T: Clone> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value,
            parent: None,
            left: None,
            right: None,
        }
    }
    
    fn flip_color(&mut self) {
        match self.color {
            NodeColor::Red => self.color = NodeColor::Black,
            NodeColor::Black => self.color = NodeColor::Red,
        }
    }
}

impl<T: PartialOrd + Clone + std::fmt::Debug + std::fmt::Display> RedBlackTree<T> {
	pub fn new() -> Self {
        RedBlackTree { 
            root: None,
            rotation_function_inner: Self::rotate_left_right,
            rotation_function_extern: Self::rotate_left_right,
        }
    }

    fn insert_recursive(parent: &mut Link<T>, value: T) -> Option<Link<T>> {
        let mut parent_mut_borrow =  parent.as_ref().borrow_mut();
        
        if parent_mut_borrow.value == value {
            return None;
        }

        let leaf = if value < parent_mut_borrow.value {
            &mut parent_mut_borrow.left
        } else {
            &mut parent_mut_borrow.right
        };

        match leaf {
            Some(ref mut node) => Self::insert_recursive(node, value),
            None => {
                let mut new_node = TreeNode::new(value);
                new_node.parent = Some(Rc::downgrade(parent));
                let inserted_node = Some(Rc::new(RefCell::new(new_node)));
                *leaf = inserted_node.clone();
                return inserted_node;
            }
        }
    }

    // 1 - Insert a node to the red-black tree.
    pub fn insert(&mut self, value: T) -> bool {
        // if the root node exist, create the mutable reference to root node
        if let Some(ref mut root) = self.root {
            if let Some(inserted_node) = Self::insert_recursive(root, value) {
                // self.print_tree();
                // Implement rotation and recoloring to maintain Red-Black Tree properties
                self.rebalance_insert(inserted_node);
            } else {
              return false;
            }
        } else {
            let mut new_node = TreeNode::new(value);
            new_node.color = NodeColor::Black;
            self.root = Some(Rc::new(RefCell::new(new_node)));
        }
        return true;
    }

    fn handle_red_uncle(&mut self, uncle_node: Rc<RefCell<TreeNode<T>>>, parent_node_ref: Rc<RefCell<TreeNode<T>>>, grand_node_ref: Rc<RefCell<TreeNode<T>>>) {
        uncle_node.borrow_mut().flip_color();
        parent_node_ref.borrow_mut().flip_color();
        grand_node_ref.borrow_mut().flip_color(); 
        self.rebalance_insert(grand_node_ref);
    }

    fn handle_other_uncle(&mut self, inserted_node_ref: Rc<RefCell<TreeNode<T>>>, parent_node_ref: Rc<RefCell<TreeNode<T>>>, grand_node_ref: Rc<RefCell<TreeNode<T>>>) {

        let inserted_node_value = inserted_node_ref.borrow_mut().value.clone();
        let grand_node_parent = grand_node_ref.borrow_mut().parent.clone();

        let mut node_values = [
            (inserted_node_value.clone(), Rc::downgrade(&inserted_node_ref)),
            (parent_node_ref.borrow_mut().value.clone(), Rc::downgrade(&parent_node_ref)),
            (grand_node_ref.borrow_mut().value.clone(), Rc::downgrade(&grand_node_ref))
        ];
        node_values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        drop(inserted_node_ref);

        let mid_node = node_values[1].1.upgrade().unwrap();
        let min_node = node_values[0].1.upgrade().unwrap();
        let max_node = node_values[2].1.upgrade().unwrap();

        let mut mid_node_borrowed = mid_node.borrow_mut();
        let mid_node_value = mid_node_borrowed.clone().value;

        let mid_node_right = mid_node_borrowed.right.clone();
        let mid_node_left = mid_node_borrowed.left.clone();

        mid_node_borrowed.left = Some(min_node);
        mid_node_borrowed.right = Some(max_node);
        mid_node_borrowed.color = NodeColor::Black;

        if let Some(left_node) = mid_node_borrowed.left.as_ref() {
            left_node.borrow_mut().parent = Some(Rc::downgrade(&mid_node));
            left_node.borrow_mut().color = NodeColor::Red;
            if mid_node_value < inserted_node_value || mid_node_value == inserted_node_value {
                left_node.borrow_mut().right = mid_node_left;
                if let Some(ref mut right_node) = left_node.borrow_mut().right.as_mut() {
                    right_node.borrow_mut().parent = Some(Rc::downgrade(&left_node));
                };
            };
        };

        if let Some(right_node) = mid_node_borrowed.right.as_mut() {
            right_node.borrow_mut().parent = Some(Rc::downgrade(&mid_node));
            right_node.borrow_mut().color = NodeColor::Red;

            if mid_node_value > inserted_node_value || mid_node_value == inserted_node_value{
                right_node.borrow_mut().left = mid_node_right;
                if let Some(ref mut left_node) = right_node.borrow_mut().right.as_mut() {
                    left_node.borrow_mut().parent = Some(Rc::downgrade(&right_node));
                };
            }
        };
        mid_node_borrowed.parent = grand_node_parent;
        
        if mid_node_borrowed.parent.is_none() {
            drop(mid_node_borrowed);
            self.root = Some(mid_node.clone());
        }else {
            if Rc::ptr_eq(&mid_node_borrowed.parent.as_ref().unwrap().upgrade().unwrap().borrow().clone().left.unwrap(), &grand_node_ref) {
                mid_node_borrowed.parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().left = Some(mid_node.clone());
            }else{
                mid_node_borrowed.parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().right = Some(mid_node.clone());
            };
            drop(mid_node_borrowed);
        };
        self.rebalance_insert(mid_node);
    }

    // calling this function after inserting operation to keep the balance of the rb tree 
    fn rebalance_insert(&mut self, inserted_node: Link<T>){
        let mut inserted_node_ref = inserted_node.borrow_mut();
        let parent = match inserted_node_ref.parent.as_mut() {
            Some(parent) => parent,
            None => { // Recursion up to the inserted_node equal the root node
                inserted_node_ref.color = NodeColor::Black;
                return;
            }
        };

        let parent_node = match parent.upgrade() {
            Some(parent_node) => parent_node,
            None => {
                println!("Parent node upgrade failed");
                return;
            }
        };

        // if the parent_node is not red, the tree is already balanced.
        if parent_node.borrow().color != NodeColor::Red || inserted_node_ref.color != NodeColor::Red{
            return;
        }

        let mut parent_node_ref = parent_node.borrow_mut();
        let grand_node = parent_node_ref.parent.as_mut().unwrap().upgrade().unwrap();
        let grand_node_ref = grand_node.borrow_mut();

        
        if let Some(left_child) = &grand_node_ref.left.clone() { // the left node of grand_node is exist.
            if Rc::ptr_eq(left_child, &parent_node) { // The parent node is the left child of the grandparent node
                drop(inserted_node_ref);
                drop(parent_node_ref);
                if let Some(uncle_node) = &grand_node_ref.right.clone() {
                    if uncle_node.borrow().color == NodeColor::Red {
                        drop(grand_node_ref);
                        self.handle_red_uncle(uncle_node.clone(), parent_node, grand_node);
                    } else {
                        // Handle the case where uncle_node is black
                    };
                } else {
                    drop(grand_node_ref);
                    self.handle_other_uncle(inserted_node, parent_node, grand_node);
                };
            }else{ // The parent node is the right child of the grandparent node
                let uncle_node = left_child;
                // println!("{}", uncle_node.borrow().value);
                if uncle_node.borrow().color == NodeColor::Red {
                    drop(inserted_node_ref);
                    drop(parent_node_ref);
                    drop(grand_node_ref);
                    self.handle_red_uncle(uncle_node.clone(), parent_node, grand_node);
                }else {
                    drop(inserted_node_ref);
                    drop(parent_node_ref);
                    drop(grand_node_ref);
                    self.handle_other_uncle(inserted_node, parent_node, grand_node);
                }
            };
        } else{ 
            drop(inserted_node_ref);
            drop(parent_node_ref);
            match &grand_node_ref.left.clone() { // try get uncle node
                Some(uncle_node) if uncle_node.borrow().color == NodeColor::Red => {
                    drop(grand_node_ref);
                    self.handle_red_uncle(uncle_node.clone(), parent_node, grand_node);
                },  
                _ => {
                    drop(grand_node_ref);
                    self.handle_other_uncle(inserted_node, parent_node, grand_node);
                },
            }
        };
    }

    fn rotate_left_right(&mut self, node: &Link<T>) {
        let mut node_borrow_mut = node.borrow_mut();
        if let Some(right) = node_borrow_mut.right.take() {
            let mut right_borrow_mut = right.borrow_mut();
            node_borrow_mut.right = right_borrow_mut.left.take();

            right_borrow_mut.left = Some(node.clone());
            if let Some(weak_parent) = node_borrow_mut.parent.as_mut() {
                if let Some(parent) = weak_parent.upgrade() {
                    let mut parent_borrow_mut = parent.borrow_mut();
                    if Rc::ptr_eq(&node, &parent_borrow_mut.left.as_mut().unwrap()){
                        parent_borrow_mut.left = Some(right.clone());
                    }else {
                        parent_borrow_mut.right = Some(right.clone());
                    }
                    right_borrow_mut.parent = Some(Rc::downgrade(&parent));
                    drop(right_borrow_mut);
                    node_borrow_mut.parent = Some(Rc::downgrade(&right));
                }
            }else {
                node_borrow_mut.parent = Some(Rc::downgrade(&right));
                right_borrow_mut.color = NodeColor::Black;
                right_borrow_mut.parent = None;
                self.root = Some(right.clone());
            }
        }
    }

    fn rotate_left_left(&mut self, node: &Link<T>) {
        let mut node_borrow_mut = node.borrow_mut();
        if let Some(right) = node_borrow_mut.right.take() {
            let mut right_borrow_mut = right.borrow_mut();
            if let Some(right_left) = right_borrow_mut.left.take(){
                let mut right_left_borrow_mut = right_left.borrow_mut();
                node_borrow_mut.right = right_left_borrow_mut.left.take();
                right_borrow_mut.left = right_left_borrow_mut.right.take();

                right_left_borrow_mut.left = Some(node.clone());
                right_left_borrow_mut.right = Some(right.clone());

                right_borrow_mut.parent = Some(Rc::downgrade(&right_left));

                if let Some(weak_parent) = node_borrow_mut.parent.as_mut() {
                    if let Some(parent) = weak_parent.upgrade() {
                        if Rc::ptr_eq(&node, &parent.borrow_mut().left.as_mut().unwrap()){
                            parent.borrow_mut().left = Some(right_left.clone());
                        }else {
                            parent.borrow_mut().right = Some(right_left.clone());
                        };
                        right_left_borrow_mut.parent = Some(Rc::downgrade(&parent));
                        drop(right_borrow_mut);
                        node_borrow_mut.parent = Some(Rc::downgrade(&right_left));
                    };
                }else {
                    node_borrow_mut.parent = Some(Rc::downgrade(&right_left));
                    right_left_borrow_mut.color = NodeColor::Black;
                    right_borrow_mut.parent = None;
                    self.root = Some(right_left.clone());
                };
            };
        };
    }

    fn rotate_right_extern(&mut self, node: &Link<T>) {
        let mut node_borrow_mut = node.borrow_mut();
        if let Some(left) = node_borrow_mut.left.take() {
            let mut left_borrow_mut = left.borrow_mut();
            node_borrow_mut.left = left_borrow_mut.right.take();

            left_borrow_mut.right = Some(node.clone());
            if let Some(weak_parent) = node_borrow_mut.parent.as_mut() {
                if let Some(parent) = weak_parent.upgrade() {
                    if Rc::ptr_eq(&node, &parent.borrow_mut().left.as_mut().unwrap()){
                        parent.borrow_mut().left = Some(left.clone());
                    }else {
                        parent.borrow_mut().right = Some(left.clone());
                    };

                    left_borrow_mut.parent = Some(Rc::downgrade(&parent));
                    drop(left_borrow_mut);
                    node_borrow_mut.parent = Some(Rc::downgrade(&left));
                }
            }else {
                left_borrow_mut.color = NodeColor::Black;
                left_borrow_mut.parent = None;
                node_borrow_mut.parent = Some(Rc::downgrade(&left));
                self.root = Some(left.clone());
            }
        }
    }

    fn rotate_right_inner(&mut self, node: &Link<T>) {
        let mut node_borrow_mut = node.borrow_mut();
        if let Some(left) = node_borrow_mut.left.take() {
            let mut left_borrow_mut = left.borrow_mut();
            if let Some(left_right) = left_borrow_mut.right.take(){
                let mut left_right_borrow_mut = left_right.borrow_mut();
                node_borrow_mut.right = left_right_borrow_mut.left.take();
                left_borrow_mut.left = left_right_borrow_mut.right.take();

                left_right_borrow_mut.left = Some(node.clone());
                left_right_borrow_mut.right = Some(left.clone());

                left_borrow_mut.parent = Some(Rc::downgrade(&left_right));

                if let Some(weak_parent) = node_borrow_mut.parent.as_mut() {
                    if let Some(parent) = weak_parent.upgrade() {
                        if Rc::ptr_eq(&node, &parent.borrow_mut().left.as_mut().unwrap()){
                            parent.borrow_mut().left = Some(left_right.clone());
                        }else {
                            parent.borrow_mut().right = Some(left_right.clone());
                        };

                        left_right_borrow_mut.parent = Some(Rc::downgrade(&parent));
                        drop(left_borrow_mut);
                        node_borrow_mut.parent = Some(Rc::downgrade(&left_right));
                    };
                }else {
                    node_borrow_mut.parent = Some(Rc::downgrade(&left_right));
                    left_right_borrow_mut.color = NodeColor::Black;
                    left_right_borrow_mut.parent = None;
                    self.root = Some(left_right.clone());
                };
            };
        };
    }

    fn delete_balanced(&mut self, node: &Link<T>) {
        let node_borrow_mut = node.borrow_mut();
        match node_borrow_mut.parent.as_ref() {
            Some(parent_weak) =>{
                let parent_node = parent_weak.upgrade().unwrap();
                let mut parent_borrow_mut = parent_node.borrow_mut();
                let parent_node_color = parent_borrow_mut.color.clone();
                let parent_node_value = parent_borrow_mut.value.clone();
                // let node_rotation: Box<dyn Fn()>;
                let node_sibling = if node_borrow_mut.value.clone() < parent_node_value {
                    self.rotation_function_inner = RedBlackTree::rotate_left_left;
                    self.rotation_function_extern = RedBlackTree::rotate_left_right;
                    &mut parent_borrow_mut.right
                } else {
                    self.rotation_function_inner = RedBlackTree::rotate_right_inner;
                    self.rotation_function_extern = RedBlackTree::rotate_right_extern;
                    &mut parent_borrow_mut.left
                };
                // get node_sibling_color and node_sibling_right_color and node_sibling_left_color. if none, the color is black;
                let mut node_sibling_borrow_mut = node_sibling.as_mut().unwrap().borrow_mut();
                let node_sibling_color = node_sibling_borrow_mut.color.clone();

                let node_sibling_left_color = node_sibling_borrow_mut.left.as_mut().map(|n| n.borrow().color.clone()).unwrap_or(NodeColor::Black);
                let node_sibling_right_color = node_sibling_borrow_mut.right.as_mut().map(|n| n.borrow().color.clone()).unwrap_or(NodeColor::Black);
                
                let flag_color;
                if node_borrow_mut.value.clone() < parent_node_value {
                    flag_color = node_sibling_left_color.clone();
                }else {
                    flag_color = node_sibling_right_color.clone();
                }
                match (parent_node_color, node_sibling_color){
                    (NodeColor::Black, NodeColor::Black) => { 
                        if node_sibling_left_color == NodeColor::Black && node_sibling_right_color == NodeColor::Black { // case 8
                            node_sibling_borrow_mut.color = NodeColor::Red;
                            drop(node_borrow_mut);
                            drop(node_sibling_borrow_mut);
                            drop(parent_borrow_mut);
                            self.delete_balanced(&parent_node);
                        }else {
                            if flag_color == NodeColor::Red { // case 5 6  [R_L] / [L_R]
                                if node_borrow_mut.value.clone() < parent_node_value {
                                    node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                    drop(node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut());
                                }else {
                                    node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                    drop(node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut());
                                }
                                drop(node_sibling_borrow_mut);
                                drop(node_borrow_mut);
                                drop(parent_borrow_mut);
                                (self.rotation_function_inner)(self, &parent_node);
                            }else { // case 7
                                if node_borrow_mut.value.clone() < parent_node_value {
                                    node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                    drop(node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut());
                                }else {
                                    node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                    drop(node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut());
                                }
                                node_sibling_borrow_mut.color = NodeColor::Black;
                                drop(node_sibling_borrow_mut);
                                parent_borrow_mut.color = NodeColor::Black;
                                drop(node_borrow_mut);
                                drop(parent_borrow_mut);
                                (self.rotation_function_extern)(self, &parent_node);                                
                            };
                        };
                    },
                    (NodeColor::Red, NodeColor::Black) => {
                        if node_sibling_left_color == NodeColor::Black && node_sibling_right_color == NodeColor::Black{ // case 3
                            node_sibling_borrow_mut.color = NodeColor::Red;
                            drop(node_sibling_borrow_mut);
                            parent_borrow_mut.color = NodeColor::Black;
                            drop(node_borrow_mut);
                            drop(parent_borrow_mut);
                        }else{
                            if flag_color == NodeColor::Red { // case 0 1
                                if node_borrow_mut.value.clone() < parent_node_value {
                                    node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut().color = NodeColor::Red;
                                    drop(node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut());
                                }else {
                                    node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut().color = NodeColor::Red;
                                    drop(node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut());
                                }
                                drop(node_sibling_borrow_mut);
                                parent_borrow_mut.color = NodeColor::Black;
                                drop(node_borrow_mut);
                                drop(parent_borrow_mut);
                                (self.rotation_function_inner)(self, &parent_node);
                            }else { // case 2
                                if node_borrow_mut.value.clone() < parent_node_value {
                                    node_sibling_borrow_mut.right.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                    // drop(node_sibling_borrow_mut.right.as_mut());
                                }else {
                                    node_sibling_borrow_mut.left.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                                    // drop(node_sibling_borrow_mut.left.as_mut());
                                }
                                node_sibling_borrow_mut.color = NodeColor::Red;
                                drop(node_sibling_borrow_mut);
                                parent_borrow_mut.color = NodeColor::Black;
                                drop(node_borrow_mut);
                                drop(parent_borrow_mut);
                                (self.rotation_function_extern)(self, &parent_node);
                            };
                        }
                    },
                    (NodeColor::Black, NodeColor::Red) => { // case 4
                        node_sibling_borrow_mut.color = NodeColor::Black;
                        drop(node_sibling_borrow_mut);
                        parent_borrow_mut.color = NodeColor::Red;
                        drop(node_borrow_mut);
                        drop(parent_borrow_mut);
                        (self.rotation_function_extern)(self, &parent_node);
                        self.delete_balanced(node);
                    },
                    (NodeColor::Red, NodeColor::Red) => {println!("Error!!")},
                };
            },
            None =>{},
        };
    }


    pub fn try_get_node(&self, value: T) -> Option<Link<T>> {
        let mut current = self.root.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return Some(node.clone());
            } else if node.borrow().value < value {
                current = node.borrow().right.clone();
            } else {
                current = node.borrow().left.clone();
            }
        }
        None
    }
    
    fn find_min_node(node: Link<T>) -> Link<T> {
        let mut current = node;
        while current.borrow().left.is_some() {
            let left = current.borrow().left.clone().unwrap();
            current = left;
        }
        current
    }

    fn delete_node(&mut self, node: Link<T>) {
        let mut node_borrow_mut = node.borrow_mut();
        match node_borrow_mut.parent.as_mut() {
            Some(parent_weak) =>{
                if let Some(parent_node) = parent_weak.upgrade() {
                    let mut parent_borrow_mut = parent_node.borrow_mut();
                    let node_self = if node_borrow_mut.value.clone() < parent_borrow_mut.value.clone() {
                        &mut parent_borrow_mut.left
                    } else {
                        &mut parent_borrow_mut.right
                    };
                    match (node_borrow_mut.left.is_some(), node_borrow_mut.right.is_some()) {
                        (true, true) => { // Node with both children
                            drop(parent_borrow_mut);
                            let successor_node = Self::find_min_node(node_borrow_mut.right.as_ref().unwrap().clone());
                            node_borrow_mut.value = successor_node.borrow().value.clone(); // swap the value
                            drop(node_borrow_mut);
                            self.delete_node(successor_node);
                        },
                        (true, false) | (false, true) => { // Node with only one child
                            *node_self = node_borrow_mut.left.take().or(node_borrow_mut.right.take());
                            let child = node_self.as_mut().unwrap();
                            child.borrow_mut().color = NodeColor::Black;
                            child.borrow_mut().parent = node_borrow_mut.parent.clone();
                        },
                        (false, false) => {
                            *node_self = None;
                            if node_borrow_mut.color.clone() == NodeColor::Black{ // double black, node_sibling must exist
                                drop(parent_borrow_mut);
                                drop(node_borrow_mut);
                                self.delete_balanced(&node);
                            };
                        },
                    };
                };
            },
            None => { // root node
                match (node_borrow_mut.left.is_some(), node_borrow_mut.right.is_some()) {
                    (true, true) => { // Root node with both children
                        let successor_node = Self::find_min_node(node_borrow_mut.right.as_ref().unwrap().clone());
                        node_borrow_mut.value = successor_node.borrow().value.clone();
                        drop(node_borrow_mut);
                        self.delete_node(successor_node);
                    },
                    (true, false) | (false, true) => { // Root node with only one child
                        let color = node_borrow_mut.color.clone();
                        self.root = node_borrow_mut.left.take().or(node_borrow_mut.right.take());
                        let mut root = self.root.clone();
                        root.as_mut().map(|child| {
                            if color == NodeColor::Black && child.borrow().color == NodeColor::Black {
                                drop(node_borrow_mut);
                                self.delete_balanced(child);
                            } else {
                                child.borrow_mut().color = NodeColor::Black;
                                child.borrow_mut().parent = None;
                            };
                        });
                    },
                    (false, false) => { // Root node with no children
                            self.root = None;
                    }, 
                }
            },
        };   
    }
    // 2 - Delete a node from the red-black tree.
    pub fn delete(&mut self, value: T) -> bool{
        if let Some(node) = self.try_get_node(value) {
            // println!("{:?}", node);
            self.delete_node(node);
            return true;
        }
        return false;
    }

    // 3- Count the number of leaves in a tree.
    pub fn get_leaves_number(&self) -> u32 {
        self.root.as_ref().map_or(0, |root| {
            let mut stack = vec![Rc::downgrade(root)];
            let mut count = 0;

            while let Some(weak_node) = stack.pop() {
                if let Some(node) = weak_node.upgrade() {
                    let node_borrow = node.borrow();
                    match (&node_borrow.left, &node_borrow.right) {
                        (None, None) => count += 1,
                        (Some(left), Some(right)) => {
                            stack.push(Rc::downgrade(left));
                            stack.push(Rc::downgrade(right));
                        },
                        (Some(left), None) => stack.push(Rc::downgrade(left)),
                        (None, Some(right)) => stack.push(Rc::downgrade(right)),
                    }
                }
            }
            count
        })
    }

    // 4 - Return the height of a tree.
    pub fn get_height(&self) -> u32 {
        let mut queue = VecDeque::new();
        let mut height = 0;

        self.root.as_ref().map(|root| queue.push_back((Rc::clone(root), 1)));

        while let Some((node, level)) = queue.pop_front() {
            height = std::cmp::max(height, level);

            let node_borrow = node.borrow();
            node_borrow.left.as_ref().map(|left| queue.push_back((Rc::clone(left), level + 1)));
            node_borrow.right.as_ref().map(|right| queue.push_back((Rc::clone(right), level + 1)));
        }

        return height;
    }

    // 5 - In-order traversal
    pub fn show_in_order_traversal(&self, visit: &dyn Fn(&T)) {
        Self::in_order_traversal_node(&self.root, visit);
    }

    fn in_order_traversal_node(node: &Option<Link<T>>, visit: &dyn Fn(&T)) {
        node.as_ref().map(|n| {
            let n_borrow = n.borrow();
            Self::in_order_traversal_node(&n_borrow.left, visit);
            visit(&n_borrow.value);
            Self::in_order_traversal_node(&n_borrow.right, visit);
        });
    }

    // 6 - Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        return self.root.is_none();
    }
    
    // 7 - Print the tree showing its colors and structure.
    fn recursion_print(node: &Option<Link<T>>, pre_space: &String, is_left: bool, child_pre: String) {
        let none_pre = if is_left { "├───" } else { "└───" };
        let pre_current = if is_left { "├───" } else { "└───" };
        let pre_child = if is_left { "|   " } else { "    " };

        node.as_ref().map(|node| {
            let node = node.borrow();
            let col = if node.color == NodeColor::Black { "Black" } else { "Red" };
            println!("{}{}{} {:?}:{}", pre_space, pre_current, child_pre, node.value, col);

            let mut pre_space = pre_space.to_owned();
            pre_space.push_str(&pre_child);

            Self::recursion_print(&node.left, &pre_space, true, "L".to_string());
            Self::recursion_print(&node.right, &pre_space, false, "R".to_string());
        }).unwrap_or_else(|| println!("{}{}{}", pre_space, none_pre, "null"));
    }


    pub fn print_tree(&self) {
        println!("\n================== TREE PRINT <Node:Color> ==================");
        Self::recursion_print(&self.root, &"".to_string(), false, "Root".to_string());
        println!("\n======================= FINISH PRINT ========================");
    }

    //  the following is some new features

    // check property 4 and 5
    fn check_properties_recursive(node: Link<T>, path_black_node_num: i32, mut needed_black_node_num: i32) -> bool {
        let mut path_black_node_num = path_black_node_num;

        if node.borrow().color == NodeColor::Black {
            path_black_node_num += 1;
        }

        // Property 5: Every path from a given node to any of its descendant NIL nodes contains the same number of black nodes
        if node.borrow().left.is_none() && node.borrow().right.is_none() {  // This is a leaf node
            if needed_black_node_num == 0 {
                // This is the first time we've reached a leaf node
                // so we set needed_black_node_num to path_black_node_num
                needed_black_node_num = path_black_node_num;
            }
            if path_black_node_num != needed_black_node_num {
                return false;
            }
        } else { // This is an internal node
            if node.borrow().color == NodeColor::Red {
                if (node.borrow().left.is_some() && node.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Red) ||
                (node.borrow().right.is_some() && node.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Red) {
                    // Property 4: If a node is red, then both its children are black
                    return false;
                }
            }
            if node.borrow().left.is_some() {
                if !Self::check_properties_recursive(node.borrow().left.as_ref().unwrap().clone(), path_black_node_num, needed_black_node_num) {
                    return false;
                }
            }
            if node.borrow().right.is_some() {
                if !Self::check_properties_recursive(node.borrow().right.as_ref().unwrap().clone(), path_black_node_num, needed_black_node_num) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn check_properties(&self) -> bool {
        self.root.as_ref().map_or(true, |root| {
            root.borrow().color == NodeColor::Black &&
            Self::check_properties_recursive(root.clone(), 0, 0)
        })
    }


    // update a node
    pub fn update_node(&mut self, old_value: T, new_value: T) -> bool{
        let delete_result = self.delete(old_value);
        let insert_result = self.insert(new_value);
        return delete_result && insert_result;
    }

    pub fn search_node(&mut self, value: T) -> bool{
        let mut current = self.root.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return true;
            } else if node.borrow().value < value {
                current = node.borrow().right.clone();
            } else {
                current = node.borrow().left.clone();
            }
        }
        return false;
    }

    // pub fn debug(&mut self){
    //     let left = &self.root.as_ref().unwrap().borrow().left;
    //     let right = &self.root.as_ref().unwrap().borrow().right;
    //     // println!("{:#?}", left);
    //     // println!("{:#?}", left.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade());

    //     let left_right = &left.as_ref().unwrap().borrow().right;
    //     let left_left = &left.as_ref().unwrap().borrow().left;
        
    //     let right_left = &right.as_ref().unwrap().borrow().left;
    //     let right_right = &right.as_ref().unwrap().borrow().right;
    //     // println!("{:#?}", right_right.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade());

    //     let right_right_left = &right_right.as_ref().unwrap().borrow().left;
    //     let right_right_right = &right_right.as_ref().unwrap().borrow().right;


    //     println!("{:#?}", right.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade());

    // }

}