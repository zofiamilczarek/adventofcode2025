#[path = "utils.rs"]
mod utils;

use crate::day7::utils::utils::lines_from_file;



fn get_new_line(prev_line: &Vec<char>, curr_line: &Vec<char>) -> Vec<char>{
    // if curr char == ^ and char_above == | then char-1 and char+1 = |
    let mut new_line = curr_line.iter().copied().collect::<Vec<char>>();
    for (i, ch) in curr_line.iter().enumerate() {
        if ch == &'^' && prev_line[i] == '|' {
            if i >= 1 {
                new_line[i-1] = '|';
            }
            if i + 1 < curr_line.len() {
                new_line[i+1] = '|';
            }
        }
        else if prev_line[i] == 'S' {
            new_line[i] = '|';
        }
    }

    new_line
}


fn run_beams(lines: Vec<Vec<char>>) -> Vec<String> {
    let nb_lines = lines.len();
    let mut new_lines = vec![lines[0].iter().copied().collect::<Vec<char>>()];

    for i in 0..(nb_lines-1) {
        new_lines.push(get_new_line(&lines[i], &lines[i+1]));
    }

    new_lines.iter().map(|line| line.iter().collect::<String>()).collect()
}


pub fn print_answer() {
    let lines = lines_from_file("data/input_day7.txt")
                                            .iter()
                                            .map(|x| x.chars().collect())
                                            .collect::<Vec<Vec<char>>>();


    println!("{}", run_beams(lines));

}