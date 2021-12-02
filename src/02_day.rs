use std::fs;

fn get_position_and_depth_product(include_aim: bool) -> i32 {
    let filename: &str = "./src/02_data.txt";
    // Open the file in read-only mode (ignoring errors).
    let rows = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    let mut aim: i32 = 0;

    for line in rows.split("\n") {
        let mut rules = line.split(' ');
        let direction = rules.next().unwrap();
        let s_amount = rules.next().unwrap();
        let amount: i32 = s_amount.parse::<i32>().unwrap();

        if include_aim {
            if direction == "up" {
                aim = &aim - amount;
            } else if direction == "down" {
                aim = &aim + amount;
            } else if direction == "forward" {
                position = position + amount;
                depth = depth + &aim * &amount;
            }
        } else {
            if direction == "up" {
                depth = depth - amount;
            } else if direction == "down" {
                depth = depth + amount;
            } else if direction == "forward" {
                position = position + amount;
            }
        }

    }
    return depth * position;
}

fn main() {

    println!("Part 1 total: {}", get_position_and_depth_product(false));  // Part 1 total: 1990000
    println!("Part 2 total: {}", get_position_and_depth_product(true));  // Part 2 total: 1975421260

}
