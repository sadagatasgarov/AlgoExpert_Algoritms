struct Node {
    name: String,
    children: Vec<Box<Node>>,
}

impl Node {
    fn depth_first_search(&self, result: &mut Vec<String>) {
        result.push(self.name.clone());

        for child in &self.children {
            child.depth_first_search(result);
        }
    }
}

fn main() {
    // Örnek ağaç yapısı
    let graph = Node {
        name: "A".to_string(),
        children: vec![
            Box::new(Node {
                name: "B".to_string(),
                children: vec![
                    Box::new(Node {
                        name: "E".to_string(),
                        children: vec![
                            Box::new(Node {
                                name: "I".to_string(),
                                children: vec![
                                    Box::new(Node { name: "J".to_string(), children: vec![] }),
                                ],
                            }),
                            Box::new(Node {
                                name: "F".to_string(),
                                children: vec![],
                            }),
                        ],
                    }),
                ],
            }),
            Box::new(Node {
                name: "C".to_string(),
                children: vec![
                    Box::new(Node { name: "D".to_string(), children: vec![] }),
                ],
            }),
            Box::new(Node {
                name: "G".to_string(),
                children: vec![
                    Box::new(Node { name: "K".to_string(), children: vec![] }),
                    Box::new(Node { name: "H".to_string(), children: vec![] }),
                ],
            }),
        ],
    };

    let mut result: Vec<String> = Vec::new();
    graph.depth_first_search(&mut result);
    println!("{:?}", result); // Çıktı: ["A", "B", "E", "I", "J", "F", "C", "D", "G", "K", "H"]
}

















// struct BinaryTree {
//     value: String,
//     left: Option<Box<BinaryTree>>,
//     right: Option<Box<BinaryTree>>,
// }



// #[derive(Debug)]
// struct Node {
//     name: String,
//     children: Vec<Box<Node>>,
// }

// impl Node {
//     fn depth_first_search(&self, mut array: Vec<String>) -> Vec<String> {
//         array.push(self.name.clone());

//         for child in &self.children {
//             array = child.depth_first_search(array);
//         }

//         array
//     }
// }


// fn main() {

//     let bst = BinaryTree {
//         value: "A".to_string(),
//         left: Some(Box::new(BinaryTree {
//             value: "C".to_string(),
//             left: Some(Box::new(BinaryTree {
//                 value: "E".to_string(),
//                 left: None,
//                 right: None,
//             })),
//             right: Some(Box::new(BinaryTree {
//                 value: "G".to_string(),
//                 left: None,
//                 right: None,
//             })),
//         })),
//         right: Some(Box::new(BinaryTree {
//             value: "K".to_string(),
//             left: None,
//             right: Some(Box::new(BinaryTree {
//                 value: "P".to_string(),
//                 left: None,
//                 right: None,
//             })),
//         })),
//     };

// }