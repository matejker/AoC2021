use std::fs;

fn is_bingo(board: [[u32; 5]; 5]) -> u32 {
    for i in 0..5 {
        if board[i].iter().sum::<u32>() == 0 {
            return board[0].iter().sum::<u32>() + board[1].iter().sum::<u32>() + board[2].iter().sum::<u32>() + board[3].iter().sum::<u32>() + board[4].iter().sum::<u32>()
        }
    }

    for j in 0..5 {
        let mut s = 0_u32;
        for i in 0..5 {
            s = s + board[i][j];
        }
        if s == 0 {
            return board[0].iter().sum::<u32>() + board[1].iter().sum::<u32>() + board[2].iter().sum::<u32>() + board[3].iter().sum::<u32>() + board[4].iter().sum::<u32>()
        }
    }

    return 0
}

fn bingo() -> u32 {
    let numbers = [
        6,69,28,50,36,84,49,13,48,90,1,33,71,0,94,59,53,58,60,96,30,34,29,91,11,41,77,95,17,80,85,
        93,7,9,74,89,18,25,26,8,87,38,68,5,12,43,27,46,62,73,16,55,22,4,65,76,54,52,83,10,21,67,15,
        47,45,40,35,66,79,51,75,39,64,24,37,72,3,44,82,32,78,63,57,2,86,31,19,92,14,97,20,56,88,81,
        70,61,42,99,23,98
    ];
    let filename: &str = "./src/04_data.txt";
    let rows = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut boards  = [[[0u32; 5]; 5]; 100];
    for (i, board) in rows.split("\n\n").enumerate() {
        for (j, row) in board.split("\n").enumerate() {
            for (k, col) in row.split(" ").enumerate() {
                boards[i][j][k] = col.parse::<u32>().unwrap();
            }
        }
    }
    let mut latest_bingo = 0_u32;
    let mut won_board = vec![];
    for n in numbers.iter() {
        for i in 0..100 {
            for j in 0..5 {
                if boards[i][j].contains(n) {
                    let index = boards[i][j].iter().position(|&r| r == *n).unwrap();
                    boards[i][j][index] = 0;
                }
            }
            let bin = is_bingo(boards[i]);
            if  bin > 0 && bin != latest_bingo && !won_board.contains(&i){
                latest_bingo = bin * n;
                won_board.push(i);
            }
        }
    }
    return latest_bingo;
}

fn main() {

    println!("Part 1 total: {}", bingo());  // Part 1 total: 71708
    println!("Part 2 total: {}", bingo());  // Part 2 total: 34726

}
