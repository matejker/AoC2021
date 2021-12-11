use std::fs;
use std::cmp;

fn count_flashes(f: [[bool; 10]; 10]) -> u32 {
    let mut n: u32 = 0;
    for i in 0..10 {
        for j in 0..10 {
            if f[i][j] {
                n = n + 1;
            }
        }
    }
    return n
}

fn dumbo_octopus(tt: usize, sync: bool) -> u32 {
    let filename: &str = "./src/11_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut octopuses: Vec<Vec<i32>> = vec![];

    for rows in content.split("\n") {
        let cells: Vec<char> = rows.chars().collect();
        let octopus: Vec<i32> = cells.iter().map(|c| *c as i32 - 0x30).collect::<Vec<_>>();
        octopuses.push(octopus);
    }

    let positions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
    ];

    let mut flashes: u32 = 0;

    for t in 0..tt {
        for i in 0..10 {
            for j in 0..10 {
                octopuses[i][j] = octopuses[i][j] + 1;
            }
        }

        let mut flash: [[bool; 10]; 10] = [[false; 10]; 10];
        let mut latest_flushes: u32 = 0;


        loop {
            for i in 0..10 {
                for j in 0..10 {
                    if octopuses[i][ j] > 9 && !flash[i][j] {
                        flash[i][j] = true;
                        for (i_s, j_s) in &positions {
                            let up: i32 = i as i32 + *i_s;
                            let left: i32 = j as i32 + *j_s;
                            if  up >= 0 && 10 > up && left >= 0 && 10 > left {
                                octopuses[up as usize][left as usize] = octopuses[up as usize][left as usize] + 1;
                            }
                        }
                    }
                }
            }
            if count_flashes(flash) == latest_flushes {
                for i in 0..10 {
                    for j in 0..10 {
                        if flash[i][j] {
                            octopuses[i][j] = 0;
                        }
                    }
                }
                flashes = flashes + latest_flushes;
                break;
            }
            latest_flushes = count_flashes(flash);
        }

        if sync && latest_flushes == 100 {
            return t as u32 + 1
        }

    }

    return flashes
}

fn main() {

    println!("Part 1 total: {}", dumbo_octopus(100, false));  // Part 1 total: 1691
    println!("Part 2 total: {}", dumbo_octopus(300, true));  // Part 2 total: 216

}
