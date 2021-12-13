use std::fs;
use std::collections::HashSet;

fn fold(plot: bool) -> usize {
    let filename: &str = "./src/13_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut s = content.split("\n\n");

    let dots_raw: &str = s.next().unwrap();
    let folds_raw: &str = s.next().unwrap();

    let mut dots = HashSet::new();

    for d in dots_raw.split("\n") {
        let mut z = d.split(",");
        let i = z.next().unwrap().parse::<u32>().unwrap();
        let j = z.next().unwrap().parse::<u32>().unwrap();

        dots.insert((i, j));
    }

    for f in folds_raw.split("\n") {
        let mut temp_dots = dots.clone();
        let v = &f[13..].parse::<u32>().unwrap();
        let e = &f[11..12];
        for d in &dots {
            if e == "x" {
                if d.0 == *v {
                    temp_dots.remove(&d);
                } else if d.0 == 2 * v {
                    temp_dots.remove(&d);
                    temp_dots.insert((0, d.1));
                } else if d.0 > *v {
                    temp_dots.remove(&d);
                    temp_dots.insert((d.0 - 2 * (d.0 % v), d.1));
                }

            } else if e == "y" {
                if d.1 == *v {
                    temp_dots.remove(&d);
                } else if d.1 == 2 * v {
                    temp_dots.remove(&d);
                    temp_dots.insert((d.0, 0));
                } else if d.1 > *v {
                    temp_dots.remove(&d);
                    temp_dots.insert((d.0, d.1 - 2 * (d.1 % v)));
                }

            }
        }


        dots = temp_dots.clone();
        if !plot {
            return dots.len();
        }
    }

    let mut output: String = "".to_owned();
    for j in 0..7 {
        for i in 0..40 {
            if dots.contains(&(i, j)) {
                output = format!("{}{}", output, "█".to_owned());
            } else {
                output = format!("{}{}", output, " ".to_owned());
            }
        }
        output = format!("{}{}", output, "\n".to_owned());
    }

    println!("{}", output);

    return dots.len()
}

fn main() {
    println!("Part 1: {}", fold(false));
    println!("Part 2: {}", fold(true));
}

// Part 1: 684
//   ██ ███  ████ ███  █     ██  █  █ █  █
//    █ █  █    █ █  █ █    █  █ █ █  █  █
//    █ █  █   █  ███  █    █    ██   ████
//    █ ███   █   █  █ █    █ ██ █ █  █  █
// █  █ █ █  █    █  █ █    █  █ █ █  █  █
//  ██  █  █ ████ ███  ████  ███ █  █ █  █
//
// Part 2: 98