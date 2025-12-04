#[path = "utils.rs"]
mod utils;
use crate::day4::utils::utils::lines_from_file;
use std::usize;

fn count_neighboring_rolls(idxes: Vec<[usize; 2]>, grid: &Vec<String>) -> i32 {
    let mut nb_neighbors = 0;

    for idx in idxes {
        if grid[idx[0] as usize].chars().nth(idx[1] as usize).unwrap() == '@' {
            nb_neighbors += 1;
        }
    }
    nb_neighbors
}

fn get_neighbour_idxes(idx: &[i32; 2], shape: &[i32; 2]) -> Vec<[usize; 2]> {
    let mut idxes: Vec<[usize; 2]> = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {

            if (idx[0] + i >= 0)
                && (idx[1] + j >= 0) // i and j dont make the idx below 0
                && (idx[0] + i < shape[0])
                && (idx[1] + j < shape[1]) // i and j dont make the idx bigger than the size of the list
                && !(i == j && i == 0) // i and j aren't both 0 (which would just add the idx itself)
            {
                idxes.push([(idx[0] + i) as usize, (idx[1] + j) as usize]);
            }
        }
    }
    idxes
}

fn get_nb_rolls(grid: &Vec<String>, max_n: i32) -> i32 {
    let mut total_nb_forkable_rolls = 0;
    let mut nb_neighboring_rolls = 0;
    let mut idx: [i32; 2];
    let shape = [grid.len() as i32, grid[0].chars().count() as i32];

    for (i, row) in (&grid).iter().enumerate() {
        for (j, item) in row.chars().enumerate() {
            if item == '@' {
                idx = [i.try_into().unwrap(), j.try_into().unwrap()];
                nb_neighboring_rolls =
                    count_neighboring_rolls(get_neighbour_idxes(&idx, &shape), grid);
                if nb_neighboring_rolls < max_n {
                    total_nb_forkable_rolls += 1;
                }
            }
        }
    }

    total_nb_forkable_rolls
}

pub fn print_answer() {
    let real_input = lines_from_file("input_day4.txt");
    // let test_input = lines_from_file("test_input_day4.txt");

    println!(
        "The number of paper rolls accessible by a fork lift (part 1) is {}",
        get_nb_rolls(&real_input, 4)
    );
}
