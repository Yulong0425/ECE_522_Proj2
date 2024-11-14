mod avl_tree;
mod redblack_tree;

use std::str::FromStr;
use std::{io, fmt::{Debug, Display}};
use redblack_tree::RedBlackTree;
use avl_tree::AVLTree;


fn handle_input() -> Option<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Cannot read input!");

    match input.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Warning: Please enter a valid integer.");
            None
        },
    }
}

//For f32 mode
fn input_to_vec<T: FromStr>() -> Result<Vec<T>, ()> {
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("Error reading input");
    if numbers.trim().is_empty() {
        println!("Warning: No input provided, please enter some integers.");
    }
    let numbers = numbers.split_whitespace();
    let mut vec: Vec<T> = Vec::new();
    for i in numbers {
        match i.parse() {
            Ok(i) => vec.push(i),
            Err(_) => println!("'{}' is not a valid floating-point number", i),
        }
    }
    Ok(vec)
}

trait GetExample {
    fn get_example_insert(&self) -> String;
    fn get_example_delete(&self) -> String;
}

impl GetExample for i32 {
    fn get_example_insert(&self) -> String{return "1 2 3 4 5".to_string();}
    fn get_example_delete(&self) -> String{return "3 4".to_string();}
}

impl GetExample for f32 {
    fn get_example_insert(&self) -> String{return "1.0 2.0 3.2 4.4 1.5".to_string();}
    fn get_example_delete(&self) -> String{return "3.2 1.5".to_string();}
}

fn avl_tree_interface<T: GetExample + Copy + Clone + Debug + PartialOrd + Display + FromStr>(type_value:T){
    println!("----------------------------------------");
    println!("Welcome for using AVL Tree!");
    println!("----------------------------------------");
    let convert_input: Box<dyn Fn() -> Result<Vec<T>, ()>> = Box::new(input_to_vec);

    let mut m_avl_tree: AVLTree<T> = AVLTree::new();

    loop {
        println!("----------------------------------------");
        println!("Please choose the operation you want to do: (input corresponding number)");
        println!("1. Insert node(s) to the tree.");
        println!("2. Delete node(s) from the tree.");
        println!("3. Count the number of leaves in a tree.");
        println!("4. Return the height of a tree.");
        println!("5. Print in-order traversal of the tree.");
        println!("6. Check if the tree is empty.");
        println!("7. Print the tree showing its colors and structure.");
        println!("8. Quit.");
        println!("----------------------------------------");
        
        let user_choice = handle_input();
        match user_choice {
            Some(1) => {
                println!("----------------------------------------");
                println!("Please input the value(s) of the node(s) that you want to insert: Separate by one whitespace. e.g. {}", type_value.get_example_insert());
                let input = convert_input().unwrap();
                let mut output = Vec::new();
                for i in input.clone() {
                    let n = m_avl_tree.search(i);
                    let mut result = false;
                    if let Some(_) = n {
                        result = true;
                    };
                    if !result {
                        m_avl_tree.insert(i);
                        output.push(i);
                    } else {
                        println!("INSERT FAILED: Node({:?}) already exists!", i);
                    };
                };
                if output.len() != 0 {
                    println!("Insert {:?} successfully.", output);
                };
            },
            Some(2) => {
                println!("----------------------------------------");
                println!("Current tree contains values above {:#?}", m_avl_tree.print_in_order());
                println!("Please input the value(s) of the node(s) that you want to delete: Separate by one whitespace in decending order. e.g. {}", type_value.get_example_delete());
                let input = convert_input().unwrap();
                for i in input.clone() {
                    let n = m_avl_tree.search(i);
                    let mut result = false;
                    if let Some(_) = n {
                        result = true;
                    };
                    if result {
                        m_avl_tree.delete(i);
                        println!("Delete {} succeed!", i);
                    }else {
                        println!("Node {} doesn't exist!", i);
                    };
                };
            },
            Some(3) => {
                println!("----------------------------------------");
                println!("The number of leaves is: {}", m_avl_tree.count_leaves());
            },
            Some(4) => {
                println!("----------------------------------------");
                println!("The height of the tree is: {}", m_avl_tree.tree_height());
            },
            Some(5) => {
                println!("----------------------------------------");
                println!("In-order traversal: ");
                m_avl_tree.print_in_order();
            },
            Some(6) => {
                println!("----------------------------------------");
                if m_avl_tree.check_empty() {
                    println!("This tree is empty");
                } else {
                    println!("This tree is not empty");
                }
            },
            Some(7) => {
                println!("----------------------------------------");
                println!("The tree structure is:");
                m_avl_tree.print_struct();
            },
            Some(8) => {
                println!("----------------------------------------");
                println!("Thank you for using!");
                break;
            },
            _ => println!("Wrong input! Input should be a number from the list, please try again..."),
        };
    };
}

fn rb_tree_interface<T: GetExample + Clone + Debug + Display + PartialOrd + FromStr + Copy>(type_value: T){
    println!("----------------------------------------");
    println!("Welcome for using Red-Black Tree!");
    println!("----------------------------------------");

    let convert_input: Box<dyn Fn() -> Result<Vec<T>, ()>> = Box::new(input_to_vec);
    let mut m_rb_tree: RedBlackTree<T> = RedBlackTree::new();

    loop {
        println!("----------------------------------------");
        println!("Please choose the operation you want to do: (input corresponding number)"); 
        println!("1. Insert node(s) to the tree.");
        println!("2. Delete node(s) from the tree.");
        println!("3. Count the number of leaves in a tree.");
        println!("4. Return the height of a tree.");
        println!("5. Print in-order traversal of the tree.");
        println!("6. Check if the tree is empty.");
        println!("7. Print the tree showing its colors and structure.");
        println!("8. Quit.");
        println!("----------------------------------------");
        let user_choice = handle_input();
        match user_choice {
            Some(1) => {
                println!("----------------------------------------");
                println!("Please input the value(s) of the node(s) that you want to insert: Separate by one whitespace. e.g. {}", type_value.get_example_insert());
                let input = convert_input().unwrap();
                let mut output = Vec::new();
                for i in input.clone() {
                    if !m_rb_tree.search_node(i) {
                        m_rb_tree.insert(i);
                        output.push(i);
                    } else {
                        println!("INSERT FAILED: Node({:?}) already exists!", i);
                    };
                };
                if output.len() != 0 {
                    println!("Insert {:?} successfully.", output);
                    println!("");
                };
            },
            Some(2) => {
                println!("----------------------------------------");
                m_rb_tree.show_in_order_traversal(&|&x: &T| print!("{} \t", x));
                println!();
                println!("Current tree contains values above");
                println!("Please input the value(s) of the node(s) that you want to delete: Separate by one whitespace in decending order. e.g. {}", type_value.get_example_delete());
                let input = convert_input().unwrap();
                for i in input.clone() {
                    if m_rb_tree.search_node(i) {
                        m_rb_tree.delete(i);
                        println!("Delete {} succeed!", i);
                    }else{
                        println!("Node {} doesn't exist!", i);
                    };
                };
            },
            Some(3) => {
                println!("----------------------------------------");
                println!("The number of leaves is: {}", m_rb_tree.get_leaves_number());
            },
            Some(4) => {
                println!("----------------------------------------");
                println!("The height of the tree is: {}", m_rb_tree.get_height());
            },
            Some(5) => {
                println!("----------------------------------------");
                println!("In-order traversal: ");
                m_rb_tree.show_in_order_traversal(&|&x: &T| print!("{} \t", x));
                println!("");
            },
            Some(6) => {
                println!("----------------------------------------");
                if m_rb_tree.is_empty() {
                    println!("This tree is empty");
                } else {
                    println!("This tree is not empty");
                };
            },
            Some(7) => {
                println!("----------------------------------------");
                println!("The tree structure is:");
                m_rb_tree.print_tree();
            },
            
            Some(8) => {
                println!("----------------------------------------");
                println!("Thank you for using!");
                break;
            },
            _ => println!("Wrong input! Input should be a number from the list, please try again..."),
        };
    };
}


fn main() {
    let mut user_choice;
    println!("=========Welcome for using AVL and Red-Black tree!=========");
    loop {
        println!("Please choose the type of tree you want to use");
        println!("1. AVL Tree");
        println!("2. Red-Black Tree");
        println!("");
        user_choice = handle_input();
        match user_choice { // Continue only if the input is either 1 or 2
                Some(1) => {
                loop {
                    println!("Please choose the type of value you want to add");
                    println!("1. Integer");
                    println!("2. Floating-point number");
                    println!("");
                    let user_choice = handle_input();
                    match user_choice {
                        Some(1) => {
                            avl_tree_interface::<i32>(1);
                            break;
                        },
                        Some(2) => {
                            avl_tree_interface::<f32>(1.0);
                            break;
                        },
                        Some(_) => println!("Please choose between 1 and 2."),
                        None => println!("No valid integer was entered."),
                    };
                };
                break;
            },
            Some(2) => {
                loop {
                    println!("Please choose the type of value you want to add");
                    println!("1. Integer");
                    println!("2. Floating-point number");
                    println!("");
                    let user_choice = handle_input();
                    match user_choice {
                        Some(1) => {
                            rb_tree_interface::<i32>(1);
                            break;
                        },
                        Some(2) => {
                            rb_tree_interface::<f32>(1.0);
                            break;
                        },
                        Some(_) => println!("Please choose between 1 and 2."),
                        None => println!("No valid integer was entered."),
                    };
                };
                break;
            },

            Some(_) => println!("Please choose between 1 and 2."),
            None => println!("No valid integer was entered."),
        }
    }
}