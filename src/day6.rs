#[path = "utils.rs"]
mod utils;

use crate::day6::utils::utils::lines_from_file;



fn get_input(lines: &Vec<String>) -> (Vec<Vec<i64>>, Vec<&str>) {
    let mut processed_line;
    let mut input: Vec<Vec<i64>> = vec![];
    for line in &lines[..(lines.len()-1)]{
        processed_line = line.split(' ')
                                  .filter(|&x| !x.is_empty())
                                  .map(|x| x.to_string().parse::<i64>().unwrap())
                                  .into_iter().collect();
        input.push(processed_line);
    }



    (input, lines[lines.len()-1].split(" ").filter(|&x| !x.is_empty()).collect())
}

fn get_input_columns(lines: &Vec<String>) -> Vec<(Vec<i64>, char)>{
    let mut curr_col;
    let mut inputs: Vec<(Vec<i64>, char)> = vec![];
    let mut input_vec = vec![];
    let mut op;

    for col in (0..lines[0].len()).rev() {
        curr_col = vec![];
        for row in 0..lines.len() {
            curr_col.push(lines[row].chars().nth(col).unwrap());
        }
        // if the col ends with an operation, construct the whole input
        if curr_col[curr_col.len()-1] == '+' || curr_col[curr_col.len()-1] == '*' {
            op = curr_col[curr_col.len()-1];
            input_vec.push(curr_col[..(curr_col.len()-1)].iter().collect::<String>());
            inputs.push((input_vec.iter()
                                      .map(|x| x.trim())
                                      .filter(|x| !x.is_empty())
                                      .map(|x| x.to_string().parse::<i64>().unwrap())
                                      .collect(), op));

            input_vec = vec![];
        }
        else {
            input_vec.push(curr_col.iter().copied().collect::<String>());
        }
    }


    inputs
}


pub fn compute(inputs: Vec<Vec<i64>> , operations: Vec<&str>) -> i64 {
    let nb_inputs = inputs.len();
    let mut tmp_res;
    let mut res = 0;
    for (col, op) in operations.iter().enumerate() {
        if op == &"*" {
            tmp_res = 1;
        }
        else {
            tmp_res = 0;
        }
        for row in 0..nb_inputs {
            if op == &"*" {
                tmp_res*=inputs[row][col];
            }
            else {
                tmp_res+=inputs[row][col];
            }
        }
        res+=tmp_res;
    }

    res
}


pub fn compute_part2(inputs: Vec<(Vec<i64>, char)>) -> i64{
    let mut sum = 0;
    let mut curr_res;
    for (input, op) in inputs {
        if op == '*' {
            curr_res = 1;
            for a in input {
                curr_res *= a;
            }
        }
        else {
            curr_res = 0;
            for a in input {
                curr_res+=a
            }
        }
        sum +=curr_res;
    }

    sum
}


pub fn print_answer() {
    let lines = lines_from_file("data/input_day6.txt");

    let (inputs, operations) = get_input(&lines);

    println!("Part 1 : result = {}", compute(inputs, operations));

    let inputs = get_input_columns(&lines);

    // println!("inputs = {:?}", inputs);

    println!("Part 2 : result = {}", compute_part2(inputs));
}