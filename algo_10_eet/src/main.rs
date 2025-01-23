struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

fn evaluateExpressionTree(tree: &BinaryTree) -> i32 {
    if tree.value >= 0 {
        return tree.value;
    }

    let lv = match &tree.left {
        Some(left) => evaluateExpressionTree(left),
        None => panic!("Left subtree is missing"),
    };

    let rv = match &tree.right {
        Some(right) => evaluateExpressionTree(right),
        None => panic!("Right subtree is missing"),
    };

    match tree.value {
        -1 => lv + rv, // Addition
        -2 => lv - rv, // Subtraction
        -3 => lv * rv, // Multiplication
        -4 => lv / rv, // Division
        _ => panic!("Invalid operator"),
    }
}

fn main() {
    // Example usage:
    let root = BinaryTree {
        value: -1, // Addition
        left: Some(Box::new(BinaryTree {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(BinaryTree {
            value: 3,
            left: None,
            right: None,
        })),
    };

    let result = evaluateExpressionTree(&root);
    println!("Result: {}", result); // Output: 5
}