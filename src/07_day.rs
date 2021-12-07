use std::fs;

fn crab_submarine(gauss: bool) -> i32 {
    let filename: &str = "./src/07_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut crabs: Vec<i32> = vec![];

    for i in content.split(",") {
        crabs.push(i.parse::<i32>().unwrap());
    }
    let min = 0_i32;
    let max = 1937_i32;
    let mut min_sum = 99999999;

    for m in min..max {
        let mut s = 0;
        for i in &crabs {
            let xi = (i - m).abs();
            if gauss {
                s = s + xi * (xi + 1) / 2;
            } else {
                s = s + xi;
            }
        }
        if s < min_sum {
            min_sum = s;
        }
    }

    return min_sum;
}

fn main() {

    println!("Part 1 total: {:?}", crab_submarine(false));  // Part 1 total: 374927
    println!("Part 2 total: {:?}", crab_submarine(true));  // Part 2 total: 1687617803407

}
