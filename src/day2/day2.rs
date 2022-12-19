use std::fs;

pub fn run() {
    let strategy_guide = read_file_lines("src/day2/input.txt");
    let score = total_score(strategy_guide);
    println!("Total score: {}", score);
}

fn read_file_lines(file_name: &str) -> Vec<String> {
    // Read the contents of the file into a string
    let contents = fs::read_to_string(file_name).expect("Failed to read file");

    // Split the string by lines
    let lines = contents.lines();

    // Convert the iterator of lines into a Vec of strings
    let mut line_vec = Vec::new();
    for line in lines {
        line_vec.push(line.trim().to_string());
    }

    return line_vec;
}

fn total_score(lines: Vec<String>) -> i32 {
    let their_moves = [("A", "Rock"), ("B", "Paper"), ("C", "Scissors")]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<_, _>>();

    let outcome = [("X", "Lose"), ("Y", "Tie"), ("Z", "Win")]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<_, _>>();

    // Initialize the total score to 0
    let mut total_score = 0;

    // Iterate over the lines in the strategy guide
    for line in lines {

        // Get the first and third characters from the iterator
        let opponent_move = their_moves[line.get(0..1).unwrap()];
        let player_move = outcome[line.get(2..3).unwrap()];

        // Calculate the score for the round
        let score = match (opponent_move, player_move) {
            ("Rock", "Lose") => 3,
            ("Paper", "Lose") => 1,
            ("Scissors", "Lose") => 2,
            ("Rock", "Tie") => 4,
            ("Paper", "Tie") => 5,
            ("Scissors", "Tie") => 6,
            ("Rock", "Win") => 8,
            ("Paper", "Win") => 9,
            ("Scissors", "Win") => 7,
            _ => panic!("this should never happen")
        };

        // Add the score for the round to the total score
        total_score += score;
    }

    // Return the total score
    total_score
}
