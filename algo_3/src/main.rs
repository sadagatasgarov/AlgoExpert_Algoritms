fn main() {
    let mut v = vec![1,2,3,4,5,6,8,10,9];
    let b = sorted_squared_array(v.clone());

    println!("{:?}", v);
    println!("{:?}", b)

}



//menim hell yolum derse baxiram gorek orada nece olur
fn sorted_squared_array(mut array: Vec<i32>) ->Vec<i32> {
    array.sort();
    let mut n = vec![];
    for i in array.iter() {
        n.push(i*i)
    }
    n
}
