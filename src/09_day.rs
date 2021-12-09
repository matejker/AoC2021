use std::fs;
use std::cmp;

fn explore_basin(origin: [usize; 2], n: usize, m: usize, mut basins: Vec<Vec<u32>>) -> usize {
    let mut next: Vec<_> = vec![origin];
    let pos: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut basin_size: usize = 1;
    loop {
        let mut o: [usize; 2] = next.pop().unwrap();
        for (i, j) in pos {
            let i_s: usize = cmp::max(0, cmp::min(n as i8, i as i8 + o[0] as i8)) as usize;
            let j_s: usize = cmp::max(0, cmp::min(m as i8, j as i8 + o[1] as i8)) as usize;
            if 0 < basins[i_s][j_s] && basins[i_s][j_s] < 9 {
                basins[i_s][j_s] = 0;
                next.push([i_s, j_s]);
                basin_size = basin_size + 1;
            }
        }
        if next.len() == 0 { break }
    }
    return basin_size
}

fn smoke_basin(second_part: bool) -> usize {
    let filename: &str = "./src/09_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut basins: Vec<Vec<u32>> = vec![];

    for rows in content.split("\r\n") {
        let row: Vec<char> = rows.chars().collect();
        let basin: Vec<u32> = row.iter().map(|c| *c as u32 - 0x30).collect::<Vec<_>>();
        basins.push(basin);
    }
    let n: usize = basins.len();
    let m: usize = basins[0].len();
    let mut local_mins: usize = 0;
    let mut basins_origins: Vec<[usize; 2]> = vec![];
    let mut basins_sizes: Vec<usize> = vec![];

    for i in 0..n {
        for j in 0..m {
            let up: usize = cmp::max(0, i as i8 - 1_i8) as usize;
            let down: usize = cmp::min(n as i8 - 1_i8, i as i8 + 1_i8) as usize;
            let left: usize = cmp::max(0, j as i8 - 1_i8) as usize;
            let right: usize = cmp::min(m as i8 - 1_i8, j as i8 + 1_i8) as usize;

            let mut is_local= basins[i][j] < basins[i][right] || (basins[i][j] == basins[i][right] && right == m - 1);
            is_local = is_local && (basins[i][j] < basins[i][left] || (basins[i][j] == basins[i][left] && left == 0));
            is_local = is_local && (basins[i][j] < basins[up][j] || (basins[i][j] == basins[up][j] && up == 0));
            is_local = is_local && (basins[i][j] < basins[down][j] || (basins[i][j] == basins[down][j] && down == n - 1));

            if is_local {
                local_mins = local_mins + basins[i][j] as usize + 1;
                basins_origins.push([i, j]);
                basins[i][j] = 0_u32;
            }
        }
    }

    if second_part {
        for o in basins_origins {
            basins_sizes.push(explore_basin(o, n - 1, m - 1, basins.clone()));
        }
        basins_sizes.sort();
        return basins_sizes.pop().unwrap() * basins_sizes.pop().unwrap() * basins_sizes.pop().unwrap()
    } else {
        return local_mins
    }
}

fn main() {

    println!("Part 1 total: {:?}", smoke_basin(false));  // Part 1 total: 508
    println!("Part 2 total: {:?}", smoke_basin(true));  // Part 2 total: 1564640

}
