use std::io::{BufReader, BufRead};
use std::fs::File;

fn char_count(x: &str, y: char) -> u32 {
    return x.matches(y).count() as u32;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut num_valid = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut splt: Vec<String> = line.split(": ").map(String::from).collect();
        let mut other_splt = splt.clone();
        let password = splt.get_mut(1).unwrap();
        let policy = other_splt.get_mut(0).unwrap();
        let mut policy_splt: Vec<String> = policy.split(" ").map(String::from).collect();
        let mut o_policy_splt = policy_splt.clone();
        let policy_char = policy_splt.get_mut(1).unwrap();
        let policy_vals = o_policy_splt.get_mut(0).unwrap();
        let policy_vals_splt: Vec<i32> = policy_vals.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let num_in_psw = char_count(&password, (&policy_char).parse().unwrap());
        let min_count = *policy_vals_splt.get(0).unwrap();
        let max_count = *policy_vals_splt.get(1).unwrap();

        let chars: Vec<_> = password.chars().collect();
        let min_char = chars.get(min_count as usize - 1).unwrap();
        let max_char = chars.get(max_count as usize - 1).unwrap();

        let mut num_matches = 0;
        if (&min_char.to_string() == policy_char) {
            num_matches += 1;
        }
        if (&max_char.to_string() == policy_char) {
            num_matches += 1;
        }

        if (num_matches == 1) {
            num_valid += 1;
        }
/*
        // part A
        if num_in_psw >= min_count as u32 && num_in_psw <= max_count as u32 {
            num_valid += 1;
        }
 */

    }
    println!("Num valid: {} ", num_valid);
}
