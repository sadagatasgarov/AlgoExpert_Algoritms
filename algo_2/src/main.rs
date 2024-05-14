fn main() {
    let v1 = vec![1, 3, 4, 5, 6];
    let v2 = vec![1, 4];
    println!("{}", validate_subsequence(v1, v2));
}

// //O(n) time | O(1) space
// fn validate_subsequence(array: Vec<i32>, sequence: Vec<i32>) -> bool {
//     let mut arr_idx = 0;
//     let mut seq_idx = 0;
//     while arr_idx < array.len() && seq_idx < sequence.len() {
//         if array[arr_idx] == sequence[seq_idx] {
//             seq_idx += 1;
//         }
//         arr_idx += 1;
//     }
//     return seq_idx == sequence.len();
// }

// //O(n) time | O(1) space
fn validate_subsequence(array: Vec<i32>, sequence: Vec<i32>) -> bool{
    let mut seq_idx = 0;
    for value in array{
        if seq_idx==sequence.len(){
            break;
        }
        if sequence[seq_idx] == value{
            seq_idx+=1;
        }
    }
    return seq_idx == sequence.len();
}
