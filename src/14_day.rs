use std::fs;
use std::collections::HashMap;

fn generate_polymer(steps: usize) -> i32 {
    let filename: &str = "./src/14_data.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut s = content.split("\n\n");
    let polymers = s.next().unwrap();
    let rules = s.next().unwrap();

    let mut rule_dict = HashMap::<&str, &str>::new();
    let mut polymers_dict = HashMap::<&str, u32>::new();

    for r in rules.split("\n") {
        let mut s = r.split(" -> ");
        let a = s.next().unwrap();
        let b = s.next().unwrap();
        rule_dict.insert(a, b);
        polymers_dict.insert(a, 0);
    }

    for i in 0..(polymers.len() - 1) {
        let p = &polymers[i..(i + 2)];
        polymers_dict.insert(p, polymers_dict.get(p).unwrap() + 1);
    }

    for _ in 0..steps {
        let mut temp_polymers_dict = HashMap::<&str, u32>::new();
        let temp = &polymers_dict.clone();
        // for (k, v) in temp.iter() {
        //     temp_polymers_dict.insert(&k.clone(), v.clone());
        // }

        for (k, v) in polymers_dict.iter() {
            if v < &1 {
                continue
            }
            let r = rule_dict.get(k).unwrap();
            let a: String = format!("{}{}", &r.to_owned(), &k[1..2].to_owned()).to_owned();
            let b: String = format!("{}{}", &k[0..1].to_owned(), &r.to_owned()).to_owned();
            polymers_dict.insert(&*a as &str, polymers_dict.get(&*a as &str).unwrap() + v);
            polymers_dict.insert(&*b as &str, polymers_dict.get(&*b as &str).unwrap() + v);

        }
    }

    println!("{:?}", polymers_dict);


    return 0;
}

fn main() {
    generate_polymer(10);

    // println!("Part 1 total: {:?}", create_graph());  // Part 1 total: 1691
    // println!("Part 2 total: {:?}", create_graph());  // Part 2 total: 216

}