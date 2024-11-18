use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
use std::fmt;

type Link<T> = Rc<RefCell<TreeNode<T>>>; 

#[derive(Clone, Debug, PartialEq)] 
pub struct TreeNode<T> {
	pub key: T,
    pub height: i32,
	pub parent: Option<Link<T>>, 
	pub left: Option<Link<T>>, 
	pub right: Option<Link<T>>,
}


impl<T: PartialOrd + Copy + std::fmt::Debug> TreeNode<T> {
	pub fn new(key: T) -> Link<T> {
		Rc::new(RefCell::new(TreeNode {
            key,
            parent: None,
            left: None,
            right: None,
            height: 1,
        }))
	}
	

    // get height of a node, could be self or others, return 0 if node is none
    fn height(node: &Option<Link<T>>) -> i32 {
        node.as_ref().map_or(0, |n| n.borrow().height)
    }

    //update height after rotation, it's 1+max(left.height, right.height)
    fn update_height(&mut self) {
        self.height = 1 + max(Self::height(&self.left), Self::height(&self.right));
    }

    // l - r, > 1 or < -1 requires rotation
    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }

    // traverse tree inorder
    fn in_order_traverse<F>(&self, visit: &F)
    where
        F: Fn(&T),
    {
        // left go first
        if let Some(ref left) = self.left {
            left.borrow().in_order_traverse(visit);
        }
        // visit self
        visit(&self.key);
        // visit right part last
        if let Some(ref right) = self.right {
            right.borrow().in_order_traverse(visit);
        }
    }

    // traverse tree preorder
    fn pre_order_traverse<F>(&self, visit: &F)
    where
        F: Fn(&T),
    {
        // visit self
        visit(&self.key);

        // left go then
        if let Some(ref left) = self.left {
            left.borrow().pre_order_traverse(visit);
        }
        
        // visit right part last
        if let Some(ref right) = self.right {
            right.borrow().pre_order_traverse(visit);
        }
    }

    // traverse tree postorder
    fn post_order_traverse<F>(&self, visit: &F)
    where
        F: Fn(&T),
    {
        // left go first
        if let Some(ref left) = self.left {
            left.borrow().post_order_traverse(visit);
        }
        
        // visit right part then
        if let Some(ref right) = self.right {
            right.borrow().post_order_traverse(visit);
        }

        // visit self last
        visit(&self.key);
    }

    fn print_structure(node: &Option<Link<T>>, depth: usize, position: &str) {
        if let Some(rc_node) = node {
            let borrowed_node = rc_node.borrow();

            // Print the current node with its depth
            println!("{}{}{:?} (Height: {})", " ".repeat(depth * 2), position, borrowed_node.key, borrowed_node.height);

            // Recursively print the left and right children, increasing the depth
            Self::print_structure(&borrowed_node.left, depth + 1, "L: ");
            Self::print_structure(&borrowed_node.right, depth + 1, "R: ");
        }
    }

    // // for debugging
    // pub fn print_info(&self) {
    //     let value = &self.key;
    //     let height = self.height;
    //     println!("value: {:?}, height: {}", value, height);
    // }
}


#[derive(Clone, PartialEq)] 
pub struct AVLTree<T> {
	root: Option<Link<T>>,
	count: u32
}

impl<T: PartialOrd + Copy + std::fmt::Debug + std::fmt::Display> AVLTree<T>  {
	pub fn new() -> Self {
        AVLTree {
            root: None,
            count: 0,
        }
    }

    // count of leaves 
    //pub fn count_leaves(&self) -> i32 {
    //    self.count.try_into().unwrap()
    //}
    pub fn count_leaves(&self) -> usize {
        fn count_leaves_recursive<T>(node: &Option<Link<T>>) -> usize {
            match node {
                Some(n) => {
                    let n_borrow = n.borrow();
                    let left_leaves = count_leaves_recursive(&n_borrow.left);
                    let right_leaves = count_leaves_recursive(&n_borrow.right);

                    // If both left and right children are None, it is a leaf node
                    if n_borrow.left.is_none() && n_borrow.right.is_none() {
                        1
                    } else {
                        left_leaves + right_leaves
                    }
                }
                None => 0,
            }
        }

        count_leaves_recursive(&self.root)
    }

    // check empty
    pub fn check_empty(&self) -> bool {
        if self.count == 0 {
            return true;
        }
        false
    }
    // height of tree 
    pub fn tree_height(&self) -> i32 {
        TreeNode::height(&self.root)
    }

    // print tree in-order
    pub fn print_in_order(&self) {
        print!("in-order: ");
        self.root.as_ref().map(|node| {
            node.borrow().in_order_traverse(&|key| {      // the node will visit value in order
                print!("{:?} ", key);
            });
        });
        println!();
    }

    // print tree pre-order
    pub fn print_pre_order(&self) {
        print!("pre-order: ");
        self.root.as_ref().map(|node| {
            node.borrow().pre_order_traverse(&|key| {      // the node will visit value in order
                print!("{:?} ", key);
            });
        });
        println!();
    }
    // print tree post-order
    pub fn print_post_order(&self) {
        print!("post-order: ");
        self.root.as_ref().map(|node| {
            node.borrow().post_order_traverse(&|key| {      // the node will visit value in order
                print!("{:?} ", key);
            });
        });
        println!();
    }

    // print structure
    pub fn print_struct(&self) {
        println!("AVL Tree Structure:");
        TreeNode::print_structure(&self.root, 0, "Root: ");
    }

    pub fn insert(&mut self, value: T) {
    	let new_node = TreeNode::new(value);
        if let Some(root) = &self.root {
            // insert node recursively
            Self::insert_recursive(&root, new_node);
            // deal root problem
            self.update_root();
            
        } else {    // empty tree
            self.root = Some(new_node);

        }
    	self.count += 1;
    }

    fn update_root(&mut self) {
        //println!("check5");
        let mut current = self.root.clone();
        // assign reference of current avoid moving value
        while let Some(ref node) = current {
            //println!("...");
            //node.borrow().print_info();
            let p = node.borrow().parent.clone();
            match p {
                Some(parent) => {
                    // if has parent, set the parent as the new current node
                    current = Some(parent);
                },
                None => {
                    // no parent
                    break;
                }
            }            
        }
        self.root = current;
    }

    fn insert_recursive(cur_node: &Link<T>, new_node: Link<T>) {
        //let mut rotate_flag: bool = false;
        
        let mut n = cur_node.borrow_mut();
        if new_node.borrow().key < n.key {
            if let Some(left) = &mut n.left.clone() {
                drop(n);    // clear borrow mut
                Self::insert_recursive(left, new_node); // Recursive call
            } else {
                new_node.borrow_mut().parent = Some(Rc::clone(cur_node));
                n.left = Some(new_node);
                drop(n);    // clear borrow mut
            }
        } else {
            if let Some(right) = &mut n.right.clone() {
                drop(n);    // clear borrow mut
                Self::insert_recursive(right, new_node); // Recursive call
            } else {
                new_node.borrow_mut().parent = Some(Rc::clone(cur_node));
                n.right = Some(new_node);
                drop(n)
            }
        }
        // re-declare n
        let mut n = cur_node.borrow_mut();
        // Update the height of the node
        n.update_height();
        // Balance the tree if necessary
        let f_balance = n.balance_factor();
        drop(n);
        // use rotation to balance the tree
        if f_balance > 1 {
            //rotate_flag = true;     // we do rotation
            // check childs balance_fact 3 2l 1ll or 3 1l 2lr (later need a left rotation first)
            let left = cur_node.borrow().left.clone();
            if let Some(old_left) =  left{ 
                if old_left.borrow().balance_factor() < 0 { 
                        // do left rotation on left and its child
                        Self::rotate_left(&old_left);
                        // change left
                        //n.left = old_left.borrow().parent;   
                }
            }
            // do right rotation on self and left child
            Self::rotate_right(cur_node);
        } 
        else if f_balance < -1 {
            //rotate_flag = true;     // we do rotation
            let right = cur_node.borrow().right.clone();
            if let Some(old_right) = right {
                if old_right.borrow().balance_factor() > 0 {    
                    // do right rotation on right and its child
                    Self::rotate_right(&old_right)
                    // change right
                    //n.right = old_right.borrow().parent;
                }
            }
            // do left rotation
            Self::rotate_left(cur_node);
        }



        //rotate_flag
    }

    // will borrow parent, currnet, right mut
    // 1 2r 3r -> 1l 2 3r
    fn rotate_left(node: &Link<T>) {
        let node_right = node.borrow_mut().right.take().unwrap();
        let node_right_left = node_right.borrow_mut().left.take();

        // put node_right_left to this left
        {
            node.borrow_mut().right = node_right_left;
            if let Some(right) = &node.borrow().right {
                right.borrow_mut().parent = Some(Rc::clone(node));
            }
        }
        // change right to root, root to left
        let p = node.borrow().parent.clone();
        if let Some(parent) = p {
            node_right.borrow_mut().parent = Some(Rc::clone(&parent));
            node_right.borrow_mut().left = Some(Rc::clone(node));
            node.borrow_mut().parent = Some(Rc::clone(&node_right));

            // change parent
            let is_left: bool = {
                let p_borrow = parent.borrow();
                if let Some(left) = p_borrow.left.as_ref() {
                    left.borrow().key == node.borrow().key
                } else {
                    false
                }
            };
            let mut p_borrow_mut = parent.borrow_mut();
            if is_left {
                p_borrow_mut.left = Some(Rc::clone(&node_right));
            } else {
                p_borrow_mut.right = Some(Rc::clone(&node_right));
            }
        } else {    // whole tree root
            node_right.borrow_mut().parent = None;
            node_right.borrow_mut().left = Some(Rc::clone(node));
            node.borrow_mut().parent = Some(Rc::clone(&node_right));
            // no modification in parent
        }
                
        // Update heights
        node.borrow_mut().update_height();
        node_right.borrow_mut().update_height();
        //println!("value: {}",node.borrow().parent.clone().unwrap().borrow().key);

    }

    fn rotate_right(node: &Link<T>) {
        let node_left = node.borrow_mut().left.take().unwrap();
        let node_left_right = node_left.borrow_mut().right.take();
        // move baby to node
        {
            node.borrow_mut().left = node_left_right;
            if let Some(left) = &node.borrow().left {
                left.borrow_mut().parent = Some(Rc::clone(node));
            }
        }
        // change root to right, left to root
        // if node has parent
        let p = node.borrow().parent.clone();
        if let Some(parent) = p {
            //drop(p);
            // inverse of rotate left
            node_left.borrow_mut().parent = Some(Rc::clone(&parent));
            node_left.borrow_mut().right = Some(Rc::clone(node));
            node.borrow_mut().parent = Some(Rc::clone(&node_left));

            // change parent
            let is_left: bool = {
                let p_borrow = parent.borrow();
                if let Some(left) = p_borrow.left.as_ref() {
                    left.borrow().key == node.borrow().key
                } else {
                    false
                }
            };
            let mut p_borrow_mut = parent.borrow_mut();
            if is_left {
                p_borrow_mut.left = Some(Rc::clone(&node_left));
            } else {
                p_borrow_mut.right = Some(Rc::clone(&node_left));
            }

        } else {        // root 
            //drop(p);
            node_left.borrow_mut().parent = None;
            node_left.borrow_mut().right = Some(Rc::clone(node));
            node.borrow_mut().parent = Some(Rc::clone(&node_left));
            // no modification in parent
        }

        // Update heights
        node.borrow_mut().update_height();
        node_left.borrow_mut().update_height();
    }


    pub fn delete(&mut self, value: T) {
        if let Some(root) = self.root.clone() {
            let (new_root, deleted) = Self::delete_recursive(root, value);
            self.root = new_root;
            
            if deleted {
                self.count -= 1;
            } else {
                println!("node not found");
            }
        }
        // deal with root changed by rotation
        self.update_root();
    }


     fn delete_recursive(node: Link<T>, value: T) -> (Option<Link<T>>, bool) {
        //println!("check {}", value);
        let mut node_borrow = node.borrow_mut();
        let mut deleted = false;

        if value < node_borrow.key {
            //println!("left");
            if let Some(left) = node_borrow.left.clone() {
                drop(node_borrow);
                let (new_left, was_deleted) = Self::delete_recursive(left, value);
                node.borrow_mut().left = new_left;
                deleted = was_deleted;
            } else {
                drop(node_borrow);  // only to drop
            }
        } else if value > node_borrow.key {
            //println!("right");
            if let Some(right) = node_borrow.right.clone() {
                drop(node_borrow);
                let (new_right, was_deleted) = Self::delete_recursive(right, value);
                node.borrow_mut().right = new_right;
                deleted = was_deleted;
            } else {
                drop(node_borrow); // only for drop
            }
        } else {
            //println!("found");
            deleted = true;
            if node_borrow.left.is_none() || node_borrow.right.is_none() {
                // Node with only one child or no child
                //println!("check 3");
                if let Some(left) = &node_borrow.left {
                    left.borrow_mut().parent = node_borrow.parent.clone();
                } else if let Some(right) = &node_borrow.right {
                    right.borrow_mut().parent = node_borrow.parent.clone();
                }
                drop(node_borrow);
                return (node.borrow().left.clone().or(node.borrow().right.clone()), true);
            } else {
                //println!("check 2");
                let in_order_successor = Self::min_value_node(node_borrow.right.clone().unwrap());
                let new_value = in_order_successor.borrow().key;
                node_borrow.key = new_value;
                drop(in_order_successor);
                drop(node_borrow);
                let old_right = node.borrow().right.clone().unwrap();
                let (new_right, _) = Self::delete_recursive(old_right, new_value);
                node.borrow_mut().right = new_right;
            }
        } 
        // Update the height of the node
        node.borrow_mut().update_height();
        // Balance the tree
        let f_balance = node.borrow().balance_factor();

        let mut new_node = node.clone();
        //drop(node_borrow);
        if f_balance > 1 {
            // check childs balance_fact 3 2l 1ll or 3 1l 2lr (later need a left rotation first)
            let left = node.borrow().left.clone();
            if let Some(old_left) =  left{ 
                if old_left.borrow().balance_factor() < 0 { 
                        // do left rotation on left and its child
                        Self::rotate_left(&old_left);
                        // change left
                        //n.left = old_left.borrow().parent;   
                }
            }
            // do right rotation on self and left child
            Self::rotate_right(&node);
            new_node = node.borrow().parent.clone().unwrap();
        } 
        else if f_balance < -1 {
            let right = node.borrow().right.clone();
            if let Some(old_right) = right {
                if old_right.borrow().balance_factor() > 0 {    
                    // do right rotation on right and its child
                    Self::rotate_right(&old_right)
                    // change right
                    //n.right = old_right.borrow().parent;
                }
            }
            // do left rotation
            Self::rotate_left(&node);
            new_node = node.borrow().parent.clone().unwrap();
        }


        (Some(new_node), deleted)
    }


    // find the node with the minimum key value in a subtree
    fn min_value_node(node: Link<T>) -> Link<T> {
        let mut current = node.clone();

        loop {
            let current_node = current.clone();
            let left = current_node.borrow().left.clone();
            match left {
                Some(left_child) => {
                    current = Rc::clone(&left_child);
                },
                None => break,
            }
        }

        current
    }
    // searching 
    pub fn search(&self, value: T) -> Option<Link<T>> {
        if let Some(node) = self.root.clone() {
            match Self::find_node(node, value) {
                Some(n) => { return Some(n); },
                None => {
                    //println!("Cannot find");
                    return None;
                }
            }
        }
        else {
            //println!("it's an empty tree");
            return None;
        }

    }
    // find node recursively
    fn find_node(node: Link<T>, value: T) ->Option<Link<T>>{
        let node_borrow = node.borrow();
        if node_borrow.key < value {
            // on the right
            if let Some(right) = &node_borrow.right {
                return Self::find_node(right.clone(), value);
            } else {
                // no right child
                return None;
            }
        } else if node_borrow.key > value {
            // on the left
            if let Some(left) = &node_borrow.left {
                return Self::find_node(left.clone(), value);
            } else {
                // no left child
                return None;
            }
        } else {
            //exact here
            drop(node_borrow);
            return Some(node);
        }
    }

    pub fn debug_format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AVL Tree Structure:\n")?;
        self.root.as_ref().map(|node| {
            self.print_structure_recursive(&Some(node.clone()), 0, f, "Root");
        });
        Ok(())
    }

    fn print_structure_recursive(
        &self,
        node: &Option<Link<T>>,
        depth: usize,
        f: &mut fmt::Formatter<'_>,
        pos: &str
    ) {
        if let Some(rc_node) = node {
            let borrowed_node = rc_node.borrow();

            // Print the current node with its depth using custom formatting
            write!(
                f,
                "{}{}: {} (Height: {})\n",
                " ".repeat(depth * 2),
                pos,
                borrowed_node.key,
                borrowed_node.height,
            )
            .unwrap();

            // Recursively print the left and right children, increasing the depth
            self.print_structure_recursive(&borrowed_node.left, depth + 1, f, "L");
            self.print_structure_recursive(&borrowed_node.right, depth + 1, f, "R");
        }
    }


    fn display_recursive(&self, node: &Option<Link<T>>, depth: usize, f: &mut fmt::Formatter<'_>, pos: &str) {
        if let Some(rc_node) = node {
            let borrowed_node = rc_node.borrow();

            // Print the current node with its key, height, and depth using custom formatting
            write!(
                f,
                "{}{}: {} (Height: {})\n",
                " ".repeat(depth * 2),
                pos,
                borrowed_node.key,
                borrowed_node.height,
            )
            .expect("Failed to write to formatter");

            // Recursively print the left and right children, increasing the depth
            self.display_recursive(&borrowed_node.left, depth + 1, f, "L");
            self.display_recursive(&borrowed_node.right, depth + 1, f, "R");
        }
    }

}


impl<T: Ord + Copy + std::fmt::Debug + std::fmt::Display> fmt::Debug for AVLTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug_format(f)
    }
}


impl<T: Ord + Copy + std::fmt::Debug + std::fmt::Display> fmt::Display for AVLTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AVL Tree Structure:\n")?;
        self.display_recursive(&self.root, 0, f, "Root");
        Ok(())
    }
}



