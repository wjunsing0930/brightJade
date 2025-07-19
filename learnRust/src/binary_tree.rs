#![allow(dead_code)]

struct TreeNode<T>{
    element : T,
    left:BinaryTree<T>,
    right:BinaryTree<T>
}

enum BinaryTree<T>{
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[test]
fn binary_tree_size(){
//size_of 用于返回一个类型所占用的 内存大小（以字节为单位）。它会告诉你这个类型在内存中占用多少字节，包含了数据本身的大小
//align_of 用于返回类型的 内存对齐要求。它告诉你类型的实例在内存中必须满足的对齐边界。
//对齐要求是指变量在内存中存放时，地址必须是某个值的倍数，通常与该类型的大小有关。
    use std::mem::{size_of, align_of};
    println!("align_of::<BinaryTree<String>>(): {}", align_of::<BinaryTree<String>>()); //8
    println!("size_of::<BinaryTree<String>>(): {}", size_of::<BinaryTree<String>>());//8
    let word = size_of::<usize>(); //堆上指针大小即为 usize大小
    println!("size_of::<usize>(): {}", size_of::<usize>());//8
    assert_eq!(size_of::<BinaryTree<String>>(), word);
    //元组
    type Triple = (&'static str, BinaryTree<&'static str>, BinaryTree<&'static str>);
    //str 字符串切片 2 usize   BinaryTree 只计算Box大小 1个usize
    assert_eq!(size_of::<Triple>(), 4 * word);
}

#[test]
fn build_binary_tree(){
    use self::BinaryTree::*;
    let jupyter_tree = NonEmpty(Box::new(TreeNode{
        element : "Jupyter",  left :Empty,  right : Empty
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode{
        element : "Mercury",  left :Empty,  right : Empty
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode{
        element: "Mars", left :jupyter_tree, right: mercury_tree,
    }));

    let venus_tree = NonEmpty(Box::new(TreeNode{
        element: "Venus", left : Empty, right:Empty
    }));

    let urans_tree = NonEmpty(Box::new(TreeNode{
        element:"Urans", left:Empty, right:venus_tree,
    }));

    let tree = NonEmpty(Box::new(TreeNode{
        element:"Saturn", left :mars_tree, right:urans_tree
    }));

    assert_eq!(tree.walk(),
        vec!["Jupyter", "Mars", "Mercury", "Saturn", "Urans", "Venus"]
    );
}
#[test]
fn test_walk(){
    let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: 1,
        left: BinaryTree::NonEmpty(Box::new(TreeNode {
            element: 2,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        })),
        right: BinaryTree::NonEmpty(Box::new(TreeNode {
            element: 3,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        })),
    }));        
    let result = tree.walk();
    println!("\ttest_walk: {:?}", result);  // 输出: [2, 1, 3]
}

impl <T:Clone> BinaryTree<T>{
    pub fn walk(&self) ->Vec<T>{
        match *self{
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) =>{
                let mut result = boxed.left.walk();
                result.push(boxed.element.clone()); //这里使用了clone,所以使用前声明Clone特征
                result.extend(boxed.right.walk());
                result
            }
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

#[test]
fn test_add_method_1() {
    let planets = vec!["Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus"];
    let mut tree = BinaryTree::Empty;
    for planet in planets {
        tree.add(planet);
    }

    assert_eq!(tree.walk(),
               vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]);
}

#[test]
fn test_add_method_2() {
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    for planet in vec!["Mars", "Jupiter", "Saturn", "Uranus"] {
        tree.add(planet);
    }

    assert_eq!(
        tree.walk(),
        vec!["Jupiter", "Mars", "Mercury", "Saturn", "Uranus", "Venus"]
    );
}

use self::BinaryTree::*;
struct TreeIter<'a, T>{
    unvisited: Vec<&'a TreeNode<T>>
}
impl<'a, T:'a> TreeIter<'a,T>{
    fn push_left_edge(&mut self, mut tree:&'a BinaryTree<T>){
        while let NonEmpty(ref node) = *tree{
            self.unvisited.push(node);
            tree = &node.left
        }
    }
}
impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}
impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}