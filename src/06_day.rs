use std::fs;

fn lanternfish(n: usize) -> u64 {
    let filename: &str = "./src/06_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut fish: [u64; 9] = [0_u64; 9];

    for index in content.split(",") {
        let i = index.parse::<usize>().unwrap();
        fish[i] = fish[i] + 1_u64;
    }

    for t in 0..n {
        fish[(t + 7_usize) % 9_usize] = fish[(t + 7_usize) % 9_usize] + fish[t % 9_usize];
    }

    return fish.iter().sum();
}

fn main() {

    println!("Part 1 total: {}", lanternfish(80));  // Part 1 total: 374927
    println!("Part 2 total: {}", lanternfish(256));  // Part 2 total: 1687617803407

}
