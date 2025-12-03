#[path = "utils.rs"] mod utils;
use std::{ops::Index, usize};

use crate::day3::utils::utils::lines_from_file;


fn min_index(array: &Vec<char>) -> usize {
    let mut i = 0;

    for (j, &value) in array.iter().enumerate() {
        if value < array[i] {
            i = j;
        }
    }

    i
}

fn get_joltage_2(bank: &String) -> i32 {
    let len = bank.len();
    let mut max_idx: usize = 0;
    let mut max1 = '0';
    
    // getting the 1st digit
    for (i, char) in bank[..(len-1)].chars().enumerate(){
        if char > max1 {
            max1 = char;
            max_idx = i;
        }
    }

    let mut max2 = bank.chars().nth(max_idx+1).unwrap();
    // getting the 2nd digit
    for (i, char) in bank[(max_idx+1)..].chars().enumerate() {
        if char > max2 {
            max2 = char;
        }
    }

    format!("{}{}", max1, max2).parse().unwrap()
}



fn get_joltage_iter(bank: &String, n: usize) -> i64 {
    let mut top_n : Vec<char> = Vec::new();
    let mut max; 
    let mut max_j= 0;
    let mut start = 0;
    let mut end = bank.len() - (n-1);
    // i : digit idx in the final result
    
    for i in 0..n {
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

fn get_joltage(bank: &String, n: usize) -> i64{
    let mut top_n : Vec<char> = Vec::new();
    let mut min_idx: usize;

    for char in bank.chars() {
        top_n.push(char);
        if top_n.iter().count() > n {
            min_idx = min_index(&top_n);
            top_n.remove(min_idx);
        }
    }

    println!("{} : {:?}", bank, top_n);

    top_n.into_iter().collect::<String>().parse::<i64>().unwrap()
}


fn get_total_joltage(battery:Vec<String>, n: usize) -> i64 {
    let mut joltage_sum = 0;
    let mut j;
    for bank in battery {
        j = get_joltage_iter(&bank, n);
        joltage_sum += j;
    }

    joltage_sum
}

pub fn print_answer() {
    let real_input = lines_from_file("../input_day3.txt");

    let test_input: Vec<String>= vec!["987654321111111","811111111111119","234234234234278","818181911112111"].iter()
                            .map(|x| x.to_string())
                            .collect();

    println!("The total joltage (part 1) is {}", get_total_joltage(real_input, 12));


}