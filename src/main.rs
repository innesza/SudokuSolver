use std::fs::File;
use std::io::{BufReader, BufRead};

struct Cell {
    super_pos: Vec<u8>,
    current_value: u32
}

fn main() {
    // Setup and initialize the board
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for i in 1..=9 {
        let mut tempRow: Vec<Cell> = Vec::new();
        for j in 1..=9 {
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
                // Clear the positions no longer possible from super_pos of cells in same row, column, and group
            }
            x += 1;
        }
        y += 1;
    }

    //println!("{}", board[0][2].current_value)
}
