fn non_constructible_change(coins: &mut Vec<i32>) -> i32 {
    coins.sort();
    let mut current_change_created = 0;
    for &coin in coins.iter() {
        if coin > current_change_created + 1 {
            return current_change_created + 1;
        }
        current_change_created += coin;
    }
    current_change_created + 1
}

fn main() {
    let mut coins = vec![5, 7, 1, 1, 2, 3, 22];
    let result = non_constructible_change(&mut coins);
    println!("Minimum non-constructible change: {:?}", result);
}