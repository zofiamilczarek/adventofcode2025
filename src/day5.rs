#[path = "utils.rs"]
mod utils;
use core::num;

use crate::day5::utils::utils::lines_from_file;



fn is_fresh(number: i64, ranges : &Vec<[i64;2]>) -> bool{
    
    for [a, b] in &ranges[1..]{
        if (a..=b).contains(&&number) {
            return true;
        }
    }

    false
}

fn get_numbers_ranges(lines : &Vec<String>) -> (Vec<i64>, Vec<[i64;2]>){
    let mut numbers = Vec::new();
    let mut ranges: Vec<[i64;2]> = Vec::new();
    let mut is_number =false;

    for line in lines{
        println!("{line}");
        if line == &"".to_string() {
            is_number = true;
        }
        else {
            if is_number {
            numbers.push(line.parse::<i64>().unwrap());
        }
        else {

            ranges.push(line.split('-')
                            .map(|x| x.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>().try_into().unwrap());
        }
        }

        
    }

    return (numbers, ranges);
}


pub fn print_answer() {

    let lines = lines_from_file("input_day5.txt");
    let numbers;
    let ranges;
    (numbers, ranges) = get_numbers_ranges(&lines);
    
    println!("{:#?}\n\n{:#?}", numbers, ranges);
}