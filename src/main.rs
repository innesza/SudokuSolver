use std::fs::File;
use std::io::{BufReader, BufRead};

struct Cell {
    super_pos: Vec<u32>,
    current_value: u32
}

// Checks if the board has been completed
fn board_complete(board: &Vec<Vec<Cell>>) -> bool {
    let mut sum: u32 = 0;
    for y in 0..=8 {
        for x in 0..=8 {
            sum += board[y][x].current_value
        }
    }
    sum == 405
}

// Checks if board is in a valid state
fn board_valid(board: &Vec<Vec<Cell>>) -> bool {
    for y in 0..=8 {
        for x in 0..=8 {
            if board[y][x].current_value == 0 && board[y][x].super_pos.len() == 0 {
                println!("{},{}", y, x);
                return false;
            }
        }
    }
    return true
}

fn print_board(board: &Vec<Vec<Cell>>) {
    for y in 0..=8 {
        for x in 0..=8 {
            print!("{}", board[y][x].current_value);
        }
        println!("");
    }
}

fn propagate(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let val: u32 = board[y][x].current_value;
    // Row
    for i in 0..=8 {
        board[y][i].super_pos.retain(|a| !a.eq(&val));
    }
    // Column
    for i in 0..=8 {
        board[i][x].super_pos.retain(|a| !a.eq(&val));
    }
    // Group
    // Gets the range of the current "group" using difference between the current value and its remainder with 3 as the minimum
    // and that value plus 2 as the maximum. This ensures it gets the full 3x3 "group"
    let minx: usize = x-(x%3);
    let miny: usize = y-(y%3);
    for i in miny..=miny+2 {
        for j in minx..=minx+2 {
            board[i][j].super_pos.retain(|a| !a.eq(&val));
        }
    }
}

fn main() {
    // Setup and initialize the board
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for _ in 1..=9 {
        let mut tempRow: Vec<Cell> = Vec::new();
        for _ in 1..=9 {
            tempRow.push(Cell{super_pos: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], current_value: 0});
        }
        board.push(tempRow)
    }
    
    // Read the puzzle in on top of initialized board
    let mut x: usize;
    let mut y: usize = 0;
    let f:BufReader<File> = BufReader::new(File::open("test.txt").expect("open failed"));
    for line in f.lines() {
        x = 0;
        for n in line.expect("Lines Failed").chars() {
            if n.to_digit(10).unwrap() != 0 {
                board[y][x].current_value = n.to_digit(10).unwrap();
                board[y][x].super_pos.clear();
                propagate(&mut board, x, y)
            }
            x += 1;
        }
        y += 1;
    }

    // Actual Solver
    let mut entropy: usize;
    loop {
        if board_complete(&board) {
            println!("Board Complete");
            break;
        }
        if !board_valid(&board) {
            println!("Invalid Board");
            // Pop top board off the stack
            // If stack is empty, puzzle is invalid?
            break;
        }
        x = 0;
        y = 0;
        entropy = 9;
        // Find lowest entropy cell (above 0)
        for i in 0..=8 {
            for j in 0..=8 {
                if board[i][j].super_pos.len() > 0 && board[i][j].super_pos.len() < entropy {
                    entropy = board[i][j].super_pos.len();
                    x = j;
                    y = i;
                }
            }
        }
        if entropy > 1 {
            // Push a copy of the board to the stack
        }
        board[y][x].current_value = board[y][x].super_pos[0];
        board[y][x].super_pos.clear();
        propagate(&mut board, x, y);
        //println!("{}", entropy);
    }

    //println!("{} at {},{}", entropy, x, y);
    //println!("{}", board_complete(board));
    print_board(&board);
    println!("{}", board[1][0].super_pos.len())

    //println!("{}", board[0][4].current_value);
    //println!("{}", board_complete(board));
    //println!("{}", board_valid(board));

    //for i in 0..=board[7][1].super_pos.len()-1 {
    //    println!("{}", board[7][1].super_pos[i]);
    //}
}
