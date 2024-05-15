fn main() {
    let competitions = vec![vec!["HTML", "C#"], vec!["HTML", "Python"], vec!["Python", "HTML"]];
    let results = vec![1, 0, 1];
    let winner = tournament_winner(competitions, results);
    println!("Tournament winner: {}", winner);
}


//O(n) time | O(k) space
fn tournament_winner(competitions: Vec<Vec<&str>>, results: Vec<i32>) -> String {
    let mut current_best_team = String::new();
    let mut scores = std::collections::HashMap::new();
    scores.insert(current_best_team.clone(), 0);

    for (idx, competition) in competitions.iter().enumerate() {
        let result = results[idx];
        let home_team = competition[0];
        let away_team = competition[1];
        let winning_team = if result == 1 { home_team } else { away_team };

        update_scores(winning_team, 3, &mut scores);

        if scores[winning_team] > scores[&current_best_team] {
            current_best_team = winning_team.to_string();
        }
    }

    current_best_team
}

fn update_scores(team: &str, points: i32, scores: &mut std::collections::HashMap<String, i32>) {
    let entry = scores.entry(team.to_string()).or_insert(0);
    *entry += points;
}
