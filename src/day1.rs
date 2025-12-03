#[path = "utils.rs"] mod utils;
use crate::day1::utils::utils::lines_from_file;

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


fn get_secret_code(rotations: &Vec<String> ) -> i32{
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

   nb_zeros
}


fn get_secret_code_part2_iter(rotations: &Vec<String>) -> i32 {
    let mut value;
    let mut sign;
    let mut nb_zeros = 0;
    let mut current_tick = 50;
    for rotation in rotations.iter() {
        value = get_rotation_value(rotation);
        sign = value.signum();
        value = value.abs();
        // println!("Rotation : {} {}", sign, value);
        for _tick in 0..value {
            current_tick = (current_tick + sign).rem_euclid(100);
            if current_tick == 0 {
                nb_zeros+=1;
            }
            // println!("  Tick : {}", current_tick);
        }
    };
    nb_zeros
}


fn get_secret_code_part2(rotations: &Vec<String>) -> i32 {
    let mut total_nb_zeros = 0;
    let mut current_tick = 50;
    let mut new_tick: i32;
    let mut value: i32;
    let mut nb_zeros = 0;

    for rotation in rotations.iter() {
        value = get_rotation_value(rotation);
        new_tick = (current_tick + value).rem_euclid(100);
        
        if value >= 0 {
            nb_zeros = (current_tick + value) / 100;
        }
        else {
            if current_tick == 0 {
                nb_zeros = value.abs() / 100;
            }
            else if value.abs() > current_tick {
                nb_zeros = ((value.abs() - current_tick - 1) / 100) + 1;
                if new_tick == 0 {
                    nb_zeros += 1;
                }
            else if value.abs() == current_tick {
                nb_zeros = 1
            }
            }
        }
        println!("start tick : {}, end tick : {}, rotation : {}, nb passses through 0: {}", current_tick, new_tick, rotation, nb_zeros);
        total_nb_zeros += nb_zeros;
        current_tick = new_tick;

    };

    total_nb_zeros
}


pub fn print_answer() {
    // let test_input : Vec<String> = vec!["L68", "L30", "R48","L5","R60","L55","L1","L99","R14","L82"]
    //                         .iter()
    //                         .map(|x| x.to_string())
    //                         .collect();

    let real_input = lines_from_file("input_day1.txt");

    let input = real_input;

    let result_1 = get_secret_code(&input);

    let result_2 = get_secret_code_part2_iter(&input);

    println!("The (simple) secret code : {}", result_1);

    println!("The (harder) secret code : {}", result_2);

}