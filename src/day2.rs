#[path = "utils.rs"] mod utils;
use crate::day2::utils::utils::lines_from_file;



fn format_input(s : &String) -> Vec<Vec<i64>> {
    s
        .split(',')
        .map(|x| x
                .split('-')
                .map(|x| x
                        .to_string()
                        .parse::<i64>().unwrap())
                        .collect())
        .collect()
}

fn is_invalid_part1(input : i64) -> bool{
    let digits_str = input.to_string();
    let nb_digits = digits_str.chars().count();
    let first_half;
    let second_half;

    if nb_digits % 2 == 0 {
        first_half = &digits_str[..(nb_digits / 2)];
        second_half = &digits_str[(nb_digits / 2)..];
        if first_half == second_half {
            return true;
        }
    }

    return false;
}

fn split_into_n_parts(digit_str: &String, n: usize) -> Vec<i64> {
    let mut parts: Vec<i64> = Vec::new();
    let mut part: i64;
    let len: usize = digit_str.chars().count();

    for i in (0..len).step_by(n) {
        part = digit_str[i..(i+n)].to_string().parse::<i64>().unwrap();

        parts.push(part);
    };

    parts
}

fn is_invalid_part2(input : i64) -> bool {
    // any repeating sequence
    let input_str = input.to_string();
    let nb_digits = input_str.chars().count();
    let mut parts: Vec<i64>;
    
    for i in  (1..=(nb_digits / 2 )).rev() {
        // you can only have equal parts if they can be of equal length
        if nb_digits % i == 0 {
            // getting a split of i parts
            parts = split_into_n_parts(&input_str, i);
            let mut all_parts_equal = parts[0] == parts[1];
            let mut prev_part = parts[1];
            for part in &parts[2..] {
                all_parts_equal = all_parts_equal && (prev_part == *part);
                prev_part = *part;

                if !all_parts_equal {
                    break;
                }
            };

            if all_parts_equal {
                return true;
            };
        };
    };

    return false;
}

fn get_invalid_inputs(start: i64, end: i64, part : i32) -> Vec<i64> {
    let mut invalid_inputs : Vec<i64> = Vec::new();
    let is_invalid;

    if part == 1 {
        is_invalid = is_invalid_part1 as fn(i64) -> bool;
    }
    else {
        is_invalid = is_invalid_part2 as fn(i64) -> bool;
    }

    for i in start..=end {
        if is_invalid(i) {
            invalid_inputs.push(i);
        }
    };

    invalid_inputs
}




fn get_invalid_inputs_sum(inputs : &String, part : i32) -> i64 {
    let formatted_inputs = format_input(inputs);
    let mut invalids : Vec<i64>;
    let mut sum = 0;
    for input in formatted_inputs{
        invalids = get_invalid_inputs(input[0], input[1], part);
        for inv in invalids {
            sum += inv;
        };
    };

    sum
}



pub fn print_answer() {

    // let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
    let real_input = lines_from_file("data/input_day2.txt")[0].clone();


    let res_part1 = get_invalid_inputs_sum(&real_input, 1);

    println!("The sum of invalid ids (part 1) : {}", res_part1); // 30323879646

    let res_part2 = get_invalid_inputs_sum(&real_input, 2);

    println!("The sum of invalid ids (part 2) : {}", res_part2);

}