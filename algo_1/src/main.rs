


// //(n^2) time | O(1) space
// fn two_number_sum(array: &Vec<i32>, targetSum:i32)->Vec<i32>{
//     //array.push(6);
//     for i in 1..array.len() {
//         let first_num = array[i];
//         for j in (i+1)..array.len() {
//             let second_num = array[j];
//             if first_num+second_num==targetSum as i32 {
//                 return vec![first_num, second_num];
//             }

//         }
//     }
//     vec![]
// }



// use std::collections::HashSet;
// //O(n) time / O(n) space
// fn two_number_sum(array: &Vec<i32>, target_sum: i32) -> Vec<i32> {
//     let mut nums=HashSet::new();
//     for num in array{
//          let potential_match = target_sum - num;
//          if nums.contains(&potential_match) {
//             return vec![potential_match, *num];
//          } else {
//              nums.insert(*num);
//          }
//     }
//     vec![]
// }

use std::cmp::max;



fn two_number_sum(array: &mut Vec<i32>, target_sum: i32) -> Vec<i32> {
    array.sort();
    let mut left = 0;
    let mut right = array.len() - 1;
    while left < right {
        let current_sum = array[left] + array[right];
        if current_sum == target_sum {
            return vec![array[left], array[right]];
        } else if current_sum < target_sum {
            left += 1;
        } else {
            right -= 1;
        }
    }
    vec![]
}

fn main() {
    let mut array = vec![3, 5, -4, 8, 11, 1, -1, 6];
    let target_sum = 10;
    let result = two_number_sum(&mut array, target_sum);
    println!("Result: {:?}", result);

    // timeConversion("12:22:22AM");
}