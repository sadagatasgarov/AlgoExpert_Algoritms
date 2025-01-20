

struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

fn branchSums(root: &BinaryTree) -> Vec<i32> {
    let mut a = Vec::new();
    calculate_branch_sum(root, 0, &mut a);
    a
}


fn calculate_branch_sum(node: &BinaryTree, running_sum: i32, sums: &mut Vec<i32>) {
    let new_running_sum =  running_sum + node.value;
    if node.left.is_none() && node.right.is_none() {
        sums.push(new_running_sum);
        return;
    }

    if let Some(left_node) = node.left.as_ref() {
        calculate_branch_sum(left_node, new_running_sum, sums);
    }

    if let Some(right_node) = node.right.as_ref() {
        calculate_branch_sum(right_node, new_running_sum, sums);
    }
}



fn main() {
    println!("Hello, world!");

    let bst = BinaryTree {
        value: 10,
        left: Some(Box::new(BinaryTree {
            value: 5,
            left: Some(Box::new(BinaryTree {
                value: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(BinaryTree {
                value: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BinaryTree {
            value: 15,
            left: None,
            right: Some(Box::new(BinaryTree {
                value: 22,
                left: None,
                right: None,
            })),
        })),
    };


    let a = branchSums(&bst);
    println!("{a:?}")

}
