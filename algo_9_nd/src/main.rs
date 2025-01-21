#[derive(Debug)]
struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

#[derive(Debug)]
struct Level<'a> {
    root: Option<&'a BinaryTree>,
    depth: usize,
}


//O(n) time | O(h) space
fn node_depths(root: Option<&BinaryTree>) -> i32 {
    let mut sum_of_depths = 0;
    let mut stack = vec![Level { root, depth: 0 }];

    while let Some(top) = stack.pop() {
        if let Some(node) = top.root {
            sum_of_depths += top.depth as i32;
            stack.push(Level { root: node.left.as_deref(), depth: top.depth + 1 });
            stack.push(Level { root: node.right.as_deref(), depth: top.depth + 1 });
        }
    }

    sum_of_depths
}

fn main() {
    // Örnek bir binary tree oluştur
    let tree = BinaryTree {
        value: 1,
        left: Some(Box::new(BinaryTree {
            value: 2,
            left: Some(Box::new(BinaryTree { value: 4, left: None, right: None })),
            right: Some(Box::new(BinaryTree { value: 5, left: None, right: None })),
        })),
        right: Some(Box::new(BinaryTree {
            value: 3,
            left: None,
            right: Some(Box::new(BinaryTree { value: 6, left: None, right: None })),
        })),
    };

    let result = node_depths(Some(&tree));
    println!("Toplam Düğümler Derinliği: {}", result);
}
