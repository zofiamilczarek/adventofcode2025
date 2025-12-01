use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_rotation_value(rotation_code: &String) -> i32  {
    
    let direction : char = rotation_code.chars().nth(0).unwrap();
    let value : i32 = rotation_code[1..].parse().unwrap();
    let mut sign : i32 = 1;
    
    if direction == 'R' {
        sign = 1;
    }
    else if direction == 'L' {
        sign = -1;
    }
    else {
        println!("Your rotation string is badly formatted")
    };

    return sign * value;
}


fn get_secret_code(rotations: Vec<String> ) -> i32{
    let mut nb_zeros = 0;
    let mut current_tick = 50;
    let mut value: i32;
    let mut new_tick: i32;
    
    for rotation in rotations.iter() {
        value = get_rotation_value(rotation);
        new_tick =( current_tick + value ) % 100;
        // println!("Rotation value : {value}");
        if new_tick == 0 {
            nb_zeros = nb_zeros + 1;
        }
        current_tick = new_tick;
    };

    return nb_zeros;
}


pub fn print_answer() {
    // let input : Vec<String> = vec!["L68", "L30", "R48","L5","R60","L55","L1","L99","R14","L82"]
    //                         .iter()
    //                         .map(|x| x.to_string())
    //                         .collect();

    let input = lines_from_file("/Users/zofia/Documents/projects/adventofcode2025/input_day1.txt");

    let result = get_secret_code(input);

    println!("The secret code : {}", result);

}