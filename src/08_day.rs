use std::fs;
use std::convert::TryInto;
use std::collections::HashSet;

fn decode_seven_segment(p: [&str; 7], d: &str) -> u32 {
    let len = d.chars().count();

    if len == 2 { return 1 }
    if len == 3 { return 7 }
    if len == 4 { return 4 }
    if len == 7 { return 8 }

    if d.contains(p[0]) && d.contains(p[1]) && d.contains(p[2]) && d.contains(p[3]) && d.contains(p[4]) && d.contains(p[6]) {
        return 9
    }
    if d.contains(p[6]) && d.contains(p[1]) && d.contains(p[2]) && d.contains(p[3]) && d.contains(p[4]) && d.contains(p[5]) {
        return 6
    }
    if d.contains(p[0]) && d.contains(p[2]) && d.contains(p[2]) && d.contains(p[3]) && d.contains(p[4]) && d.contains(p[6]) {
        return 0
    }
    if d.contains(p[0]) && d.contains(p[1]) && d.contains(p[3]) && d.contains(p[4]) && d.contains(p[6]) {
        return 5
    }
    if d.contains(p[0]) && d.contains(p[1]) && d.contains(p[2]) && d.contains(p[5]) && d.contains(p[6]) {
        return 3
    }
    if d.contains(p[0]) && d.contains(p[2]) && d.contains(p[3]) && d.contains(p[5]) && d.contains(p[6]) {
        return 2
    }

    return 0
}

fn get_pattern(signal: &str) -> [&str; 7] {
    let pattern: [&str; 7] = ["a", "b", "c", "d", "e", "f", "g"];
    let mut digits = [
        HashSet::new(), HashSet::new(), HashSet::new(),
        HashSet::new(), HashSet::new(), HashSet::new(),
        HashSet::new(), HashSet::new(), HashSet::new(),
        HashSet::new()
    ];

    for i in 0..10 {
        for s in signal.split(" ") {
            let len = s.chars().count();
            let a: HashSet::<_> = s.split("").collect();

            if digits[1].is_empty() && len == 2 {
                digits[1] = a.clone(); // one
            } else if digits[7].is_empty() && len == 3 {
                digits[7] = a.clone(); // seven
            } else if digits[4].is_empty() && len == 4 {
                digits[4] = a.clone(); // four
            } else if digits[8].is_empty() && len == 7 {
                digits[8] = a.clone(); // eight
            } else if digits[9].is_empty() && len == 6 && a.intersection(&digits[4]).count() == 5 {
                digits[9] = a.clone(); // nine
            } else if digits[2].is_empty() && len == 5 && a.intersection(&digits[4]).count() == 3 {
                digits[2] = a.clone(); // two
            } else if digits[6].is_empty() && len == 6 && a.union(&digits[1]).count() == 8 {
                digits[6] = a.clone(); // six
            } else if !digits[6].is_empty() && digits[0].is_empty() && len == 6 {
                digits[0] = a.clone(); // zero
            } else if digits[5].is_empty() && len == 5 && a.difference(&digits[9]).count() == 0 && digits[7].difference(&a).count() == 1 {
                digits[5] = a.clone(); // five
            } else if digits[3].is_empty() && len == 5 && a.difference(&digits[9]).count() == 0 && digits[7].difference(&a).count() == 0 {
                digits[3] = a.clone(); // three
            }

        }
    }
    println!("{:?}", digits);
    let union: HashSet<&str> = digits[7].union(&digits[4]).collect();

    println!("1: {:?}", digits[7].difference(&digits[1]));
    println!("2: {:?}", digits[4].difference(&digits[3]));
    println!("3: {:?}", digits[1].difference(&digits[6]));
    println!("4: {:?}", digits[8].difference(&digits[0]));
    println!("5: {:?}", digits[8].difference(&digits[9]));
    println!("6: {:?}", digits[7].difference(&digits[2]));
    println!("7: {:?}", digits[9].difference(&union));

    return pattern
}

fn seven_segment_search_part_one() -> u32 {
    let filename: &str = "./src/08_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut mun_easy = 0_u32;

    for equation in content.split("\n") {
        let mut s = equation.split(" | ");
        let signal = s.next().unwrap();
        let output = s.next().unwrap();
        let pattern = ["a", "b", "c", "d", "e", "f", "g"];

        for o in output.split(" ") {
            let d = decode_seven_segment(pattern,o);
            if [1, 4, 7, 8].contains(&d) {
                mun_easy = mun_easy + 1;
            }
        }
    }


    return mun_easy;
}

fn seven_segment_search_part_two() -> u32 {
    let filename: &str = "./src/08_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut mun_easy = 0_u32;

    for equation in content.split("\n") {
        let mut s = equation.split(" | ");
        let signal = s.next().unwrap();
        let output = s.next().unwrap();
        let pattern = get_pattern(signal);

        for (i, o) in output.split(" ").enumerate() {
            let d = decode_seven_segment(pattern, o);
            mun_easy = mun_easy + d * 10_u32.pow((3 - i as u32).try_into().unwrap());
            println!("{}", &d * 10_u32.pow((3 - i as u32).try_into().unwrap()));
        }
    }


    return mun_easy;
}

fn main() {

    println!("Part 1 total: {:?}", seven_segment_search_part_one());  // Part 1 total: 374927
    println!("Part 2 total: {:?}", seven_segment_search_part_two());  // Part 2 total: 1687617803407

}
