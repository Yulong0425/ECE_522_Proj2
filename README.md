# Group Members: 
| Name      | ID      |
| ------------- | ------------- |
| Yicheng Yang | 1647241 |
| Jiale Cai | 1815497 |
| Zhouyiyang Yang | 1868452 |
| Yulong Zhang | 1823084 |
___

# Design Document
Dear valued user, we appreciate your decision to utilize our AVL and Red-Black tree libraries.
<br>
The purpose of this library is to provide APIs that allows users to create memory efficient AVL and Red-Black tree. 
Besides, by using this library, users can investigate the performance difference between AVL and Red-Black tree, which helps them deeply understand the algorithms.
<br>

## Table of Contents
- [Overview](#overview)
- [Major Innovations](#major-innovations)
    - [The Support of Multiple Data Types](#the-support-of-multiple-data-types)
    - [Friendly Interactive System](#friendly-interactive-system)
- [Performance Evaluation](#performance-evaluation)
- [System Limitations](#system-limitations)
- [User Manual](#system-limitations)
    - [Base Operations](#base-operations)


## Overview
Our libraries offer two types of self-balancing binary search trees: AVL and Red-Black tree. Both of these data structures are designed to maintain their balance following insertions and deletions, allowing for efficient search operations.

- **AVL Tree Library**: \
The AVL Tree library provides an implementation of AVL Trees, named after their inventors Adelson-Velsky and Landis. AVL trees are known for their rigid balance, ensuring a height difference of at most one between the left and right child nodes. This characteristic guarantees a worst-case lookup, insertion, and deletion time complexity of O(log n).

- **Red-Black Tree Library**: \
The Red-Black Tree library offers an implementation of Red-Black Trees, a type of self-balancing binary search tree where each node contains an extra bit for denoting the color of the node, either red or black. By constraining the node colors on any simple path from the root to a leaf, Red-Black trees ensure that no path is more than twice as long as any other, resulting in a balanced structure with good worst-case guarantees for its operations.

<div style="page-break-after: always;"></div>

## Major Innovations
### The Support of Multiple Data Types
Experience the power of versatility with our innovative AVL and Red-Black tree libraries in Rust! These libraries are designed to support multiple data types, providing you with the flexibility to manage not just integers or strings, but also complex data structures. Whether you're developing a simple application or a complex system, our libraries adapt to your needs, broadening their usability across different scenarios. 
<br>
Leveraging Rust's powerful features like generics and traits, we've overcome the challenges of a statically typed language Leveraging Rust's powerful features like generics and traits, we've overcome the challenges of a statically typed language to offer you a solution that ensures type safety and performance.

### Friendly Interactive System
Our libraries come with a user-friendly interactive system that simplifies the process of working with AVL and Red-Black trees. This system allows you to perform operations like insertion, deletion, and search with just a few commands, making it easier for you to focus on your application logic rather than the intricacies of tree management.

## Performance Evaluation
The evaluation criteria are as follows:
```sudo
    for tree_size in (10,000, 40,000, 70,000, 100,000, 130,000) do:
        Start by creating an empty tree.
        Values with tree_size are inserted into the tree.
        A search is conducted for the (tree_size/10) lowest values.
    end
```

This evaluation was done on the following computer configurations, and the results may vary from computer to computer.
> AMD Ryzen 5 3600 6-Core CPU / 16G DDR4 3200MHz Memory

We use criterion create to perform the benchmark. For more information, please click [here](https://crates.io/crates/criterion).

<div style="text-align: center">
    <img src="image/Design%20Document/result_summary.png" style="width:95%" /> <br>
    <div style="color:orange; border-bottom: 1px solid #d9d9d9; display: inline-block; color: #999; padding: 2px;">Result Summary Table</div>
</div>

<div style="text-align: center">
    <img src="image/Design%20Document/Insert Benchmark Result.png" style="width:85%" /> <br>
    <div style="color:orange; border-bottom: 1px solid #d9d9d9; display: inline-block; color: #999; padding: 2px;">Insert Benchmark Result</div>
</div>

<div style="text-align: center">
    <img src="image/Design%20Document/Search Benchmark Result.png" style="width:85%" /> <br>
    <div style="color:orange; border-bottom: 1px solid #d9d9d9; display: inline-block; color: #999; padding: 2px;">Search Benchmark Result</div>
</div>

Experience the power of efficiency with our AVL Tree, the champion of insertion performance! 
And when it comes to search operations, nothing beats our RB Tree. 
Both of them outshine the competition, setting a new standard in performance.

## System Limitations
- Inadequate error-capture system
- Mixed data types are not supported
- Unfixed bugs in RedBlackTree Deletion, we will continually fix it in the weekend

## User Manual
For a quick start, you can simply use `cargo run` to enter the interactive system.
Here is the ouput you might see.

```
    =========Welcome for using AVL and Red-Black tree!=========
    Please choose the type of tree you want to use
    1. AVL Tree
    2. Red-Black Tree
```

After that, you can type `1` or `2` to choose the type of tree you want to use.
```
    Please choose the type of value you want to add
    1. Integer
    2. Floating-point number
```

After choosing the type of value, you can take the operation to the tree. Here is the output for AVL and Red-Black tree.
```
    ----------------------------------------
    Welcome for using AVL Tree!
    ----------------------------------------
    ----------------------------------------
    Please choose the operation you want to do: (input corresponding number)
    1. Insert node(s) to the tree.
    2. Delete node(s) from the tree.
    3. Count the number of leaves in a tree.
    4. Return the height of a tree.
    5. Print in-order traversal of the tree.
    6. Check if the tree is empty.
    7. Print the tree showing its colors and structure.
    8. Print pre-order traversal of the tree.
    9. Print post-order traversal of the tree.
    10. Check if the element(s) exists in the tree.
    11. Quit.
    ----------------------------------------
```

```
    ----------------------------------------
    Welcome for using Red-Black Tree!
    ----------------------------------------
    ----------------------------------------
    Please choose the operation you want to do: (input corresponding number)
    1. Insert node(s) to the tree.
    2. Delete node(s) from the tree.
    3. Count the number of leaves in a tree.
    4. Return the height of a tree. 
    5. Print in-order traversal of the tree.
    6. Check if the tree is empty.
    7. Print the tree showing its colors and structure.
    8. Check tree's property.
    9. Update the value of a specific node.
    10. Check if the element(s) exists in the tree.
    11. Quit.
    ----------------------------------------
```

### Base Operations
The base operations from 1 to 7 are essentially identical for both trees. Therefore, I will only display the output for one of them.

1. Insert node(s) to the tree. you can type `1.1 4.2 6.4 6.5 7.5 10.1 9.8` to insert seven nodes to the tree.
```
    ----------------------------------------
    1
    ----------------------------------------
    Please input the value(s) of the node(s) that you want to insert: Separate by one whitespace. e.g. 1.0 2.0 3.2 4.4 1.5
    1.1 4.2 6.4 6.5 7.5 10.1 9.8
    Insert [1.1, 4.2, 6.4, 6.5, 7.5, 10.1, 9.8] successfully.
    ----------------------------------------
```


2. Delete node(s) from the tree. you can type `4.2 6.4` to delete node `4.2` and node `6.4` from the tree.
```
    ----------------------------------------
    2
    ----------------------------------------
    in-order: 1.1 4.2 6.4 6.5 7.5 9.8 10.1 
    Current tree contains values above ()
    Please input the value(s) of the node(s) that you want to delete: Separate by one whitespace in decending order. e.g. 3.2 1.5
    4.2 6.4
    Delete 4.2 succeed!
    Delete 6.4 succeed!
    ----------------------------------------
```

3. Count the number of leaf nodes in the tree.
```
    ----------------------------------------
    3
    ----------------------------------------
    The number of leaves is: 3
    ----------------------------------------
```

4. Return the height of a tree.
```
    ----------------------------------------
    4
    ----------------------------------------
    The height of the tree is: 3
    ----------------------------------------
```

5. Print in-order traversal of the tree.
```
    ----------------------------------------
    5
    ----------------------------------------
    In-order traversal: 
    in-order: 1.1 6.5 7.5 9.8 10.1 
    ----------------------------------------
```

6. Check if the tree is empty.
```
    ----------------------------------------
    6
    ----------------------------------------
    This tree is not empty
    ----------------------------------------
```

7. Print the tree showing its structure.
```
    ----------------------------------------
    7
    ----------------------------------------
    The tree structure is:
    AVL Tree Structure:
    Root: 6.5 (Height: 3)
    L: 1.1 (Height: 1)
    R: 9.8 (Height: 2)
        L: 7.5 (Height: 1)
        R: 10.1 (Height: 1)
    ----------------------------------------
```
### Feature Operations
There are certain feature operations that differ between the AVL and Red-Black tree.
- AVL Tree <br>
    8.  Print pre-order traversal of the tree.
    ```
        ----------------------------------------
        8
        ----------------------------------------
        Pre-order traversal: 
        pre-order: 6.5 1.1 9.8 7.5 10.1 
        ----------------------------------------
    ```
    9. Print post-order traversal of the tree.
    ```
        ----------------------------------------
        9
        ----------------------------------------
        Post-order traversal: 
        post-order: 1.1 7.5 10.1 9.8 6.5 
        ----------------------------------------
    ```
    10. Check if the element(s) exists in the tree.
    ```
        ----------------------------------------
        10
        ----------------------------------------
        Please input the node(s) you want to check. Separate by one whitespace.
        5.0 1.1
        Node: 5.0 Existence: false
        Node: 1.1 Existence: true
        ----------------------------------------
    ```

- Red-Black Tree <br>
    8. Check each property of the Red-Black tree.
    ```
        ----------------------------------------
        8
        ----------------------------------------
        The current tree satisfies each attribute? true
        ----------------------------------------
    ```
    9. Update the value of a specific node.
    ```
        ----------------------------------------
        9
        ----------------------------------------
        1.1     6.5     7.5     9.8     10.1 
        Current tree contains values above.
        Please input the node you want to update. Separate by one whitespace e.g. 1 2(replace 1 with 2)
        7.5 2.2
        Update succeed!
        ----------------------------
        1.1     2.2     6.5     9.8     10.1 
        Current tree contains values above.
        ----------------------------------------
    ```
    10. Check if the element(s) exists in the tree.
    ```
        ----------------------------------------
        10
        ----------------------------------------
        Please input the node(s) you want to check. Separate by one whitespace.
        2.2 6.6
        Node: 2.2 Existence: true
        Node: 6.6 Existence: false
        ----------------------------------------
    ```

### Benchmark
Run the Benchmark using
```shell
$ cargo bench
```
