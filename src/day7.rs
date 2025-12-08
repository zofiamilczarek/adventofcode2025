#[path = "utils.rs"]
mod utils;

use crate::day7::utils::utils::lines_from_file;



fn get_new_line_and_nb_splits(prev_line: &Vec<char>, curr_line: &Vec<char>) -> (Vec<char>, i32){
    let mut new_line = curr_line.iter().copied().collect::<Vec<char>>();
    let mut nb_splits = 0;
    for (i, ch) in curr_line.iter().enumerate() {
        // if split
        if ch == &'^' && prev_line[i] == '|' {
            if i >= 1 {
                new_line[i-1] = '|';
            }
            if i + 1 < curr_line.len() {
                new_line[i+1] = '|';
            }
            nb_splits+=1;
        }
        else if prev_line[i] == 'S' || prev_line[i] == '|' {
            new_line[i] = '|';
        }
    }

    (new_line, nb_splits)
}


fn run_beams(lines: Vec<Vec<char>>) -> (Vec<String>, i32) {
    let nb_lines = lines.len();
    let mut new_lines = vec![lines[0].iter().copied().collect::<Vec<char>>()];
    let mut total_nb_splits = 0;
    let mut nb_splits;
    let mut new_line;

    for i in 0..(nb_lines-1) {
        (new_line, nb_splits) = get_new_line_and_nb_splits(&new_lines[i], &lines[i+1]);
        new_lines.push(new_line);
        total_nb_splits += nb_splits;
    }

    (new_lines.iter().map(|line| line.iter().collect::<String>()).collect(), total_nb_splits)
}


pub fn print_answer() {
    let lines = lines_from_file("data/input_day7.txt")
                                            .iter()
                                            .map(|x| x.chars().collect())
                                            .collect::<Vec<Vec<char>>>();

    let (_beams, nbsplits) = run_beams(lines);
    println!("The beam split {} times", nbsplits);

}