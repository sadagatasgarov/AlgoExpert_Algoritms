fn main() {
    let mut v = vec![1, -8, 2, 3, 4, 5, 6, 8, 10, 9];
    let b = sorted_squared_array(v.clone());

    println!("{:?}", v);
    println!("{:?}", b)
}

// O(n) time | O(n) space
fn sorted_squared_array(mut arr: Vec<i32>) -> Vec<i32>{
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = vec![0; arr.len()];

    for idx in (0..arr.len()).rev() {
        println!("{}", idx);
        let smaller_value = arr[left];
        let larger_value = arr[right];

        if smaller_value.abs() >= larger_value.abs() {
            result[idx] = smaller_value * smaller_value;
            left += 1;
        } else {
            result[idx] = larger_value * larger_value;
            right -= 1;
        }
    }
    // Copy theht result back to the original array
    arr.copy_from_slice(&result);
    arr
}



// //O(nlogn) time | O(n) space
// fn sorted_squared_array(array: Vec<i32>) -> Vec<i32> {
//     let mut sorted = vec![0; array.len()];
//     for (idx, &value) in array.iter().enumerate() {
//         sorted[idx] = value * value;
//     }
//     sorted.sort();
//     sorted
// }

// // iteratorlar ile helli
// fn sorted_squared_array(mut array: Vec<i32>) -> Vec<i32> {
//     array.iter_mut().for_each(|value| *value *= *value);
//     array.sort();
//     array
// }

// //menim hell yolum derse baxiram gorek orada nece olur
// fn sorted_squared_array(array: Vec<i32>) ->Vec<i32> {
//     let mut n = vec![];
//     for i in array.iter() {
//         n.push(i*i)
//     }
//     n.sort();
//     n
// }
