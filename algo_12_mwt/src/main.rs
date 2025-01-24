// O(nlogn) time | O(1) space
fn minimum_waiting_time(mut query: Vec<usize>) -> usize {
    query.sort(); // Diziyi küçükten büyüğe sırala
    let mut total_waiting_time = 0; // Toplam bekleme süresi

    for (idx, duration) in query.iter().enumerate() {
        let remaining_queries = query.len() - (idx + 1); // Geriye kalan işlemler
        total_waiting_time += duration * remaining_queries; // Bekleme süresine katkı ekle
    }

    total_waiting_time
}

fn main() {
    let v = vec![3, 2, 1, 8];

    let mwt = minimum_waiting_time(v);

    println!("Minimum Waiting Time: {}", mwt); // Çıktı: 10
}
