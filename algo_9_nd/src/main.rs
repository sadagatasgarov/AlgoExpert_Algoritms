


struct BinaryTree {
    value: i32,
    left_node: Option<Box<BinaryTree>>,
    right_node: Option<Box<BinaryTree>>
}

fn main() {

    let bt = BinaryTree{
        value:1,
        left_node: Some(Box::new(BinaryTree{
            value: 2,
            left_node: None,
            right_node: Some(Box::new(BinaryTree{
                value: 3,
                left_node: None,
                right_node: None,
            }))
        })),
        right_node:None
    };

}
