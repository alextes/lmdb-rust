use avl_tree::AVLTree;

mod avl_tree {
    #[derive(Debug)]
    struct Node<T> {
        value: T
    }
    #[derive(Debug)]
    pub struct AVLTree<T> {
        root: Option<Node<T>>
    }

    impl<T> AVLTree<T> {
        pub fn new() -> AVLTree<T> {
            AVLTree { root: None }
        }
    }
}

fn main() {
    let store: AVLTree<String> = AVLTree::new();
    println!("{:?}", store);
}
