#[path = "utils.rs"]
mod utils;
use core::num;

use crate::day5::utils::utils::lines_from_file;



fn is_fresh(number: i64, ranges : &Vec<[i64;2]>) -> bool{
    
    for [a, b] in ranges{
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



fn get_nb_fresh(numbers: &Vec<i64>, ranges : &Vec<[i64;2]>) -> i32 {
    let mut nb_fresh = 0;
    for n in numbers {
        if is_fresh(*n, ranges) {
            nb_fresh +=1;
        }
    }
    nb_fresh
}

fn map_bool_to_status(b: bool) -> String {
    if b {
        return "fresh".to_string();
    }
    else {
        return "spoiled".to_string();
    }
}


fn sort_ranges(ranges : &Vec<[i64;2]>) -> Vec<[i64;2]> {
    let mut sorted = ranges.clone();

    sorted.sort_by(|a, b| a[0].cmp(&b[0]));

    sorted
}


fn merge_2ranges(range1: [i64;2], range2: [i64;2]) -> Option<[i64;2]> {
    // if overlpping -> merge and return
    // else -> return an error
}


pub fn print_answer() {
    // let lines = lines_from_file("input_day5.txt");
    let lines = lines_from_file("test_input_day5.txt");
    let numbers;
    let ranges;
    (numbers, ranges) = get_numbers_ranges(&lines);
    
    println!("There are {} fresh ingredient ids", get_nb_fresh(&numbers, &ranges));

    println!("Sorted ranges :: {:?}", sort_ranges(&ranges))
}