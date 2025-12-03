#[path = "utils.rs"] mod utils;
use std::{usize};
use crate::day3::utils::utils::lines_from_file;


fn get_joltage(bank: &String, n: usize) -> i64 {
    let mut top_n : Vec<char> = Vec::new();
    let mut max; 
    let mut max_j= 0;
    let mut start = 0;
    let mut end = bank.len() - (n-1);
    // i : digit idx in the final result
    
    for _i in 0..n {
        max = '0';
        // j : idx of a digit in bank
        for j in start..end{
            if (&bank).chars().nth(j).unwrap() > max {
                max = (&bank).chars().nth(j).unwrap();
                max_j = j;
            }
        }
        start = max_j + 1;
        end += 1;
        top_n.push(max);

    }

    top_n.into_iter().collect::<String>().parse::<i64>().unwrap()
}


fn get_total_joltage(battery:&Vec<String>, n: usize) -> i64 {
    let mut joltage_sum = 0;
    let mut j;
    for bank in battery {
        j = get_joltage(&bank, n);
        joltage_sum += j;
    }

    joltage_sum
}

pub fn print_answer() {
    let real_input = lines_from_file("../input_day3.txt");

    // let test_input: Vec<String>= vec!["987654321111111","811111111111119","234234234234278","818181911112111"].iter()
    //                         .map(|x| x.to_string())
    //                         .collect();

    println!("The total joltage (part 1) is {}", get_total_joltage(&real_input, 2));

    println!("The total joltage (part 2) is {}", get_total_joltage(&real_input, 12));


}