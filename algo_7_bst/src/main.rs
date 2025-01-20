// struct BST {
//     value: i32,
//     left: Option<Box<BST>>,
//     right: Option<Box<BST>>,
// }

// impl BST {
//     // Public method to find the closest value
//     fn find_closest_value(&self, target: i32) -> i32 {
//         self.find_closest_value_helper(target, self.value)
//     }

//     // Avarage: O(log(n)) time | O(log(n)) space
//     // Worst: O(n) time | O(n) space
//     // Recursive helper method
//     fn find_closest_value_helper(&self, target: i32, closest: i32) -> i32 {
//         let current_closest = if (target - closest).abs() > (target - self.value).abs() {
//             self.value
//         } else {
//             closest
//         };

//         if target < self.value {
//             if let Some(left) = &self.left {
//                 return left.find_closest_value_helper(target, current_closest);
//             }
//         } else if target > self.value {
//             if let Some(right) = &self.right {
//                 return right.find_closest_value_helper(target, current_closest);
//             }
//         }
//         current_closest
//     }
// }

// fn main() {
//     // Example: Creating a BST
//     let bst = BST {
//         value: 10,
//         left: Some(Box::new(BST {
//             value: 5,
//             left: Some(Box::new(BST {
//                 value: 2,
//                 left: None,
//                 right: None,
//             })),
//             right: Some(Box::new(BST {
//                 value: 5,
//                 left: None,
//                 right: None,
//             })),
//         })),
//         right: Some(Box::new(BST {
//             value: 15,
//             left: None,
//             right: Some(Box::new(BST {
//                 value: 22,
//                 left: None,
//                 right: None,
//             })),
//         })),
//     };

//     let target = 12;
//     let closest_value = bst.find_closest_value(target);
//     println!("The closest value to {} is {}", target, closest_value);
// }

struct BST {
    value: i32,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}


// FindClosestValue iteratively with O(n) time and O(1) space
impl BST {
    // Find the closest value iteratively
    fn find_closest_value(&self, target: i32) -> i32 {
        let mut closest = self.value;
        let mut current = Some(self);

        while let Some(node) = current {
            if (target - closest).abs() > (target - node.value).abs() {
                closest = node.value;
            }
            if target < node.value {
                current = node.left.as_deref();
            } else if target > node.value {
                current = node.right.as_deref();
            } else {
                break; // Exact match found
            }
        }

        closest
    }
}

fn main() {
    // Example: Create BST
    let bst = BST {
        value: 10,
        left: Some(Box::new(BST {
            value: 5,
            left: Some(Box::new(BST {
                value: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(BST {
                value: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(BST {
            value: 15,
            left: None,
            right: Some(Box::new(BST {
                value: 22,
                left: None,
                right: None,
            })),
        })),
    };

    let target = 12;
    let closest_value = bst.find_closest_value(target);
    println!("The closest value to {} is {}", target, closest_value);
}


