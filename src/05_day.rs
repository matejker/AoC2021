use std::fs;
// use std::collections::HashSet;
// use std::iter::FromIterator;

fn hydrothermal_vents() -> usize {
    let filename: &str = "./src/05_data.txt";
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut points = vec![];
    let mut double_points = vec![];

    for line in lines.split("\n") {
        let mut p = line.split(" -> ");
        let start_coor = p.next().unwrap();
        let end_coor = p.next().unwrap();

        let (mut s, mut e) = (start_coor.split(","), end_coor.split(","));

        let x1 = s.next().unwrap().parse::<i32>().unwrap();
        let y1 = s.next().unwrap().parse::<i32>().unwrap();

        let x2 = e.next().unwrap().parse::<i32>().unwrap();
        let y2 = e.next().unwrap().parse::<i32>().unwrap();

        let mut y_increments = 0;
        let mut x_increments = 0;

        if y1 < y2 {
            y_increments = 1;
        } else if y1 > y2 {
            y_increments = -1;
        }

        if x1 < x2 {
            x_increments = 1;
        } else if x1 > x2 {
            x_increments = -1;
        }

        if x_increments != 0 && y_increments != 0 {
            let d = (y1 - y2).abs() + 1_i32;
            for i in 0..d {
                let p = &(x1 + i as i32 * &x_increments, y1 + i as i32 * &y_increments);
                if points.contains(p) && !double_points.contains(p) {
                    double_points.push(*p);
                }
                points.push(*p);
            }
        } else if x_increments == 0 && y_increments != 0 {
            // vertical
            if y_increments > 0 {
                for j in y1..(y2 + 1_i32) {
                    if points.contains(&(x1, j)) && !double_points.contains(&(x1, j)) {
                        double_points.push((x1, j));
                    }
                    points.push((x1, j));
                }
            } else {
                for j in y2..(y1 + 1_i32) {
                    if points.contains(&(x1, j)) && !double_points.contains(&(x1, j)) {
                        double_points.push((x1, j));
                    }
                    points.push((x1, j));
                }
            }

        } else if x_increments != 0 && y_increments == 0 {
            // horizontal
            if x_increments > 0 {
                for j in x1..(x2 + 1_i32) {
                    if points.contains(&(j, y1)) && !double_points.contains(&(j, y1)) {
                        double_points.push((j, y1));
                    }
                    points.push((j, y1));
                }
            } else {
                for j in x2..(x1 + 1_i32) {
                    if points.contains(&(j, y1)) && !double_points.contains(&(j, y1)) {
                        double_points.push((j, y1));
                    }
                    points.push((j, y1));
                }
            }
        }

    }
    // println!("{:?}",HashSet::<&(i32, i32)>::from_iter(&double_points));
    // return HashSet::<(i32, i32)>::from_iter(double_points).len();
    return double_points.len()
}

fn main() {

    // println!("Part 1 total: {}", hydrothermal_vents());  // Part 1 total:
    println!("Part 2 total: {}", hydrothermal_vents());  // Part 2 total: 21140

}
