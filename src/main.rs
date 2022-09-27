use std::fs::File;
use std::io;

struct Cell {
    super_pos: Vec<u8>,
    current_value: u8,
    group: u8
}

// Based on the coordinates of the current cell, return which 3x3 grid group the cell is in
fn get_group(row: i32, col: i32) -> u8 {
    0
}

fn main() {
    // Setup and initialize the board
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for i in 1..=10 {
        let mut tempRow: Vec<Cell> = Vec::new();
        for j in 1..=10 {
            tempRow.push(Cell{super_pos: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], current_value: 0, group: get_group(i, j)});
        }
        board.push(tempRow)
    }

    //println!("{}", board[1][1].current_value)
}
