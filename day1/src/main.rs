use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut entries = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let entry = line.parse::<i32>().unwrap();
        entries.push(entry);
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
    }

    let other_entries = entries.clone();
    let other_entries_2 = entries.clone();
    for &entry in &entries {
        for &other_entry in &other_entries {
            for &other_entry_2 in &other_entries_2 {
                if entry + other_entry + other_entry_2 == 2020 {
                    println!("{} {} {} {}", entry, other_entry, other_entry_2, entry * other_entry * other_entry_2);
                    return;
                }
            }
        }
    }
}