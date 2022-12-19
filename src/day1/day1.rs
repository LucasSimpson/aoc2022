use std::fs;

pub fn run() {
    // Read the file into a string
    let file_contents = fs::read_to_string("src/day1/input.txt").expect("Failed to read file");

    // Split the string on newline characters to get a vector of strings
    let int_strings = file_contents.split("\n");

    // Parse each string into an integer and return a vector of integers
    let mut ints: Vec<i32> = Vec::new();
    for string in int_strings {
        if string.trim().is_empty() {
            // If the string is empty, insert a 0 into the vector
            ints.push(0);
        } else {
            // Otherwise, parse the string into an integer and insert it into the vector
            let int = string.trim().parse().expect(&*format!("Failed to parse string as integer: {}", string));
            ints.push(int);
        }
    }

    // Step 1
    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut current_elf: Vec<i32> = Vec::new();

    // Step 2
    for calorie in ints {
        // Step 3
        if calorie == 0 {
            elves.push(current_elf);
            current_elf = Vec::new();
        } else {
            // Step 4
            current_elf.push(calorie);
        }
    }

    // Step 5
    let mut max_calories = 0;
    let mut max_elf = 0;
    for (i, elf) in elves.iter().enumerate() {
        let total_calories = elf.iter().sum();
        if total_calories > max_calories {
            max_calories = total_calories;
            max_elf = i;
        }
    }

    // Step 6
    println!("Elf {} is carrying the most calories: {}", max_elf, max_calories);

    let mut by_cal: Vec<i32> = elves.iter().map(|c|  c.iter().sum() ).collect();

    // Sort the vector in descending order
    by_cal.sort_by(|a, b| b.cmp(a));

    // Sum the first three elements of the vector
    let sum = by_cal.iter().take(3).fold(0, |acc, x| acc + x);

    // Print the sum
    println!("{}", sum);
}
