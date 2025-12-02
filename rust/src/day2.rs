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

fn get_invalid_inputs(start: i64, end: i64) -> Vec<i64> {
    let mut digits_str : String;
    let mut nb_digits : usize;
    let mut first_half;
    let mut second_half;
    let mut invalid_inputs : Vec<i64> = Vec::new();

    for i in start..=end {
        digits_str = i.to_string();
        nb_digits = digits_str.chars().count();
        if nb_digits % 2 == 0 {
            first_half = &digits_str[..(nb_digits / 2)];
            second_half = &digits_str[(nb_digits / 2)..];
            if first_half == second_half {
                invalid_inputs.push(i);
            }
        }
    };

    invalid_inputs
}


fn get_invalid_inputs_sum(inputs : String) -> i64 {
    let formatted_inputs = format_input(&inputs);
    let mut invalids : Vec<i64>;
    let mut sum = 0;
    for input in formatted_inputs{
        invalids = get_invalid_inputs(input[0], input[1]);
        for inv in invalids {
            sum += inv;
        };
    };

    sum
}



pub fn print_answer() {

    // let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
    let real_input = lines_from_file("/Users/zofia/Documents/projects/adventofcode2025/input_day2.txt")[0].clone();


    let final_res = get_invalid_inputs_sum(real_input);

    println!("The sum of invalid ids (part 1) : {}", final_res);


}