#[path = "utils.rs"]
mod utils;
use crate::day8::utils::utils::lines_from_file;
use core::f64;
use std::collections::HashMap;

fn euclidean_dist(a: &Vec<i64>, b: &Vec<i64>) -> f64 {
    let mut dist: f64 = 0.0;
    for (i, ai) in a.iter().enumerate() {
        dist += ((ai-b[i])^2) as f64;
    }

    return dist.sqrt();
}


fn get_hashmaps(coordinates: &Vec<Vec<i64>>) -> (HashMap<usize, Vec<i64>>, HashMap<Vec<i64>, usize>) {
    let mut id2circuit: HashMap<usize, Vec<i64>>= HashMap::new();
    let mut circuit2id: HashMap<Vec<i64>, usize> = HashMap::new();

    let mut c_formatted: Vec<i64>;
    for (i, c) in coordinates.iter().enumerate() {

        id2circuit.insert(i, c.iter().copied().collect());
        circuit2id.insert(c.iter().copied().collect(), i);
    }

    (id2circuit, circuit2id)
}

fn get_closest(c: &Vec<i64>, coordinates: &Vec<Vec<i64>>) -> Vec<i64>{
    let mut min_dist = f64::MAX;
    let mut dist;
    let mut closest_coords=Vec::new();
    for c_comp in coordinates {
        if c != c_comp {
            dist = euclidean_dist(c, c_comp);
            if dist < min_dist {
                min_dist = dist;
                closest_coords = c_comp.to_vec();
            }
        }
    }

    closest_coords
}

fn get_nb_circuits(coordinates: &Vec<Vec<i64>> ) {

    let (mut id2circuit, mut circuit2id) = get_hashmaps(coordinates);

    let mut closest_coords;

    for c in coordinates {
        closest_coords = get_closest(c, coordinates);
        // Q? : what to do when multiple elements are closesrt to c?
        // if closest coords empty -> skip
        // else -> remove the circuits of c and closets_coords and create a joined circuit from the 2
    }

    // return nb elements in id2circuit

}

pub fn print_answer() {
    let coordinates = lines_from_file("data/input_day8.txt")
                                            .iter()
                                            .map(|x| x.split(",")
                                                    .map(|x| x.to_string()
                                                        .parse::<i64>().unwrap()
                                                        ).collect()
                                            )
                                            .collect::<Vec<Vec<i64>>>();
    let (x, y) = get_hashmaps(&coordinates);
    println!("{:#?}, {:#?}", x, y);

}