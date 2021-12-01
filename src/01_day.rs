use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_num_increased(offset: usize) -> i32 {
    let filename: &str = "./src/01_data.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_increased: i32 = -1;
    let mut memory = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse::<i32>().unwrap(); // Ignore errors.
        memory.insert(i, 0);
        for j in 0..offset {
            if j <= i {
                memory[i - j] = memory[i - j] + line;
            }
        }
    }

    let mut previous_row: i32 = 0;
    for line in &memory {
        if previous_row < *line {
            total_increased = total_increased + 1;
        }
        previous_row = *line;
    }
    return total_increased;
}

fn main() {

    println!("Part 1 total increased: {}", get_num_increased(1));  // Part 1 total increased: 1553
    println!("Part 2 total increased: {}", get_num_increased(3));  // Part 2 total increased: 1553

}
