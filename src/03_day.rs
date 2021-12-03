use std::fs;
use std::fs::File;
use std::convert::TryInto;
use std::io::{BufRead, BufReader};

fn get_power_consumption() -> u32 {
    let filename: &str = "./src/03_data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut memory: [u32; 12] = [0; 12];
    let num_lines: u32 = 500;

    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect(); // Ignore errors.
        for i in 0..12 {
            memory[i] = memory[i] + line[i] as u32 - 0x30;
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..12 {
        if memory[i] > num_lines {
            gamma = gamma + 2_u32.pow((11 - i as u32).try_into().unwrap());
        } else {
            epsilon = epsilon + 2_u32.pow((11 - i as u32).try_into().unwrap());
        }
    }

    return epsilon * gamma;
}

fn get_life_support_rating() -> u32 {
    let filename: &str = "./src/03_data.txt";
    let rows = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut memory_a: [u32; 12] = [0; 12];
    let mut memory_b: [u32; 12] = [0; 12];

    let mut num_lines_a: f32 = 0.0;
    let mut num_lines_b: f32 = 0.0;

    let mut oxygen_generator = 0;
    let mut co2_scrubber = 0;

    for i in 0..12 {
        num_lines_a = 0.0;
        num_lines_b = 0.0;
        for line in rows.split("\n") {
            let mut ca: bool = true;
            let mut cb: bool = true;

            for j in 0..i {
                ca = ca && (memory_a[j] == line[j..j+1].parse::<u32>().unwrap());
                cb = cb && (memory_b[j] == line[j..j+1].parse::<u32>().unwrap());
            }
            if ca {
                num_lines_a = num_lines_a + 1.0;
                memory_a[i] = memory_a[i] + &line[i..i+1].parse::<u32>().unwrap();
            }
            if cb {
                num_lines_b = num_lines_b + 1.0;
                memory_b[i] = memory_b[i] + &line[i..i+1].parse::<u32>().unwrap();
            }
        }

        if num_lines_a == 1.0 {
            oxygen_generator = oxygen_generator + 2_u32.pow((11 - i as u32).try_into().unwrap()) * memory_a[i];
        } else if memory_a[i] as f32 >= num_lines_a / 2.0 {
            oxygen_generator = oxygen_generator + 2_u32.pow((11 - i as u32).try_into().unwrap());
            memory_a[i] = 1;
        } else {
            memory_a[i] = 0;
        }

        if num_lines_b == 1.0 {
            co2_scrubber = co2_scrubber + 2_u32.pow((11 - i as u32).try_into().unwrap()) * memory_b[i];
        } else if memory_b[i] as f32 >= num_lines_b / 2.0 {
            memory_b[i] = 0;
        } else {
            memory_b[i] = 1;
            co2_scrubber = co2_scrubber + 2_u32.pow((11 - i as u32).try_into().unwrap());
        }
    }

    return co2_scrubber * oxygen_generator;
}

fn main() {

    println!("Part 1: {}", get_power_consumption());  // Part 1: 1997414
    println!("Part 2: {}", get_life_support_rating());  // Part 2: 1032597

}
