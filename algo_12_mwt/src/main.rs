
// O(nlogn) time | O(1) space
fn minimum_waiting_time(mut query: Vec<usize>) -> usize {
    query.sort();
    let mut total_waiting_time = 0;

    for (idx, duration) in query.iter().enumerate(){
        let qleft = query.len() - (idx);
        total_waiting_time += duration * qleft;      //println!("{}, {:?}, {}", qleft, duration, total_waiting_time)
    }

    total_waiting_time
}



fn main() {
    let v = vec![3, 2, 1, 8];

    let mwt = minimum_waiting_time(v);

    println!("{}", mwt);
}
