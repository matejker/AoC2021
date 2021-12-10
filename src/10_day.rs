use std::fs;

fn cost(b: char) -> u64 {
    if b == ">".chars().next().unwrap() { return 25137 }
    else if b == "}".chars().next().unwrap() { return 1197 }
    else if b == "]".chars().next().unwrap() { return 57 }
    else if b == ")".chars().next().unwrap() { return 3 }

    return 0
}

fn syntax_scoring() -> u64 {
    let filename: &str = "./src/10_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let opening: Vec<char> = "([{<".chars().collect();
    let closing: Vec<char> = ")]}>".chars().collect();
    let mut error_score: u64 = 0;
    for rows in content.split("\r\n") {
        let row: Vec<char> = rows.chars().collect();
        let mut heap: Vec<char> = vec![];

        for b in row {
            if opening.contains(&b) {
                heap.push(b);
            } else if closing.contains(&b) {
                let i = closing.iter().position(|&r| r == b).unwrap();
                let c = heap.pop().unwrap();
                if opening[i] != c {
                    error_score = error_score + cost(b);
                }
            }
        }
    }


    return error_score
}

fn part2() -> u64 {
    let filename: &str = "./src/10_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let opening: Vec<char> = "([{<".chars().collect();
    let closing: Vec<char> = ")]}>".chars().collect();

    let mut scores: Vec<u64> = vec![];

    for rows in content.split("\r\n") {
        let row: Vec<char> = rows.chars().collect();
        let mut heap: Vec<char> = vec![];

        for b in row {
            if opening.contains(&b) {
                heap.push(b);
            } else if closing.contains(&b) {
                let i = closing.iter().position(|&r| r == b).unwrap();
                let c = heap.pop().unwrap();
                if opening[i] != c {
                    break;
                }
            }
        }

        let mut score: u64 = 0;

        loop {
            let c = heap.pop().unwrap();
            let r_c = opening.iter().position(|&r| r == c).unwrap();
            score = 5 * score + r_c as u64 + 1;
            println!("{} {}", c, score);
            if heap.len() == 0 {
                break;
            }
        }

        scores.push(score);

    }

    println!("{:?}", scores);

    return 0
}

fn main() {

    println!("Part 1 total: {}", syntax_scoring());  // Part 1 total: 387363
    println!("Part 2 total: {}", part2());  // Part 2 total:

}
