use std::{vec, array};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use indicatif::ProgressBar;
use rayon::prelude::*;
use smallvec::*;

const N_PUZZLES: u64 = 1_000_000_000;

type Grid = [[u8; 9]; 9];

fn main() {
    let bar = ProgressBar::new(N_PUZZLES);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}/{eta_precise}] {wide_bar} {per_sec} {pos:>7}/{len:7} {msg}")
            .unwrap()
    );
    (0..N_PUZZLES).into_par_iter().for_each(|_| {
        let puzzle = make_puzzle();
        print_sudoku(&puzzle);
        let answer = solve(puzzle).unwrap();
        print_sudoku(&answer);
        println!("-----------------");
        bar.inc(1);
    });
    bar.finish();
}

fn make_puzzle() -> Grid {
    let blank = [[0; 9]; 9];
    let mut puzzle = solve(blank).unwrap();
    let mut rng = thread_rng();
    loop {
        // Try removing a random element from the puzzle
        let row = rng.gen_range(0..9);
        let col = rng.gen_range(0..9);
        let mut new_puzzle = puzzle.clone();
        new_puzzle[row][col] = 0;
        // If it doesn't result in a random sodoku grid, then just return the
        // puzzle from before
        if !unique_solution(&puzzle) {
            break;
        }
        puzzle = new_puzzle;
    }
    return puzzle;
}

// Returns true if the given puzzle has only one unique solution, and is
// therefore a valid Sudoku puzzle.
fn unique_solution(puzzle: &Grid) -> bool {
    // Solve the puzzle non-deterministically 10 times and make sure all the
    // solutions are the same.
    let solution = solve(puzzle.clone());
    for i in 0..10 {
        let new_solution = solve(puzzle.clone());
        if new_solution != solution {
            return false;
        }
    }
    return true;
}

fn duplicate(v: &[u8]) -> bool {
    for i in 0..v.len() {
        for j in i+1..v.len() {
            if  v[i] != 0 && v[i] == v[j] {
                return true;
            }
        }
    }
    return false;
}

fn is_valid(grid: &Grid) -> bool {
    // Check that no rows or columns contain duplicate nonzero elements
    for i in 0..9 {
        let row = &grid[i];
        let col: Vec<u8> = (0..9).map(|j| grid[j][i]).collect();
        if duplicate(row) || duplicate(&col) {
            return false;
        }
    }
    // Check that no 3x3 blocks contain duplicate nonzero elements
    for row in (0..9).step_by(3) {
        for col in (0..9).step_by(3) {
            let mut found = [false; 9];
            for i in 0..3 {
                for j in 0..3 {
                    let num: usize = grid[row+i][col+j] as usize;
                    if num > 0 {
                        if found[num - 1] {
                            return false;
                        }
                        found[num - 1] = true;
                    }
                    // block.push(grid[row+i][col+j]);
                }
            }
            // if duplicate(&block) {
            // if found.iter().any(|&x| !x) {
            //     return false;
            // }
        }
    }
    return true;
}

// Solves the sudoku puzzle with recursive backtracking.
fn solve(puzzle: Grid) -> Option<Grid> {
    // Find the row and col of the last blank space (marked with a zero)
    // print_sudoku(&puzzle);
    let mut row = 0;
    let mut col = 0;
    let mut full = true;
    for i in 0..9 {
        for j in 0..9 {
            if puzzle[i][j] == 0 {
                row = i;
                col = j;
                full = false;
            }
        }
    }
    // Base case: If everything is full, return the grid if it's valid, or None
    if full {
        if is_valid(&puzzle) {
            return Some(puzzle);
        } else {
            return None;
        }
    }
    // Recursive step: try all numbers 1 through 9 in the blank space and recurse
    // Try numbers 1 through 9 in random order to break determinism
    let mut numbers: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);
    for i in numbers {
        let mut new_grid = puzzle.clone();
        new_grid[row][col] = i;
        if is_valid(&new_grid) {
            let solution = solve(new_grid);
            if solution.is_some() {
                return solution;
            }
        }
    }
    // If no valid solution is found, we return None
    return None;
}

fn print_sudoku(grid: &Grid) {
    for row in 0..9 {
        for col in 0..9 {
            print!("{} ", grid[row][col]);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solver() {
        let puzzle = [
            [0, 0, 0, 2, 6, 0, 7, 0, 1],
            [6, 8, 0, 0, 7, 0, 0, 9, 0],
            [1, 9, 0, 0, 0, 4, 5, 0, 0],
            [8, 2, 0, 1, 0, 0, 0, 4, 0],
            [0, 0, 4, 6, 0, 2, 9, 0, 0],
            [0, 5, 0, 0, 0, 3, 0, 2, 8],
            [0, 0, 9, 3, 0, 0, 0, 7, 4],
            [0, 4, 0, 0, 5, 0, 0, 3, 6],
            [7, 0, 3, 0, 1, 8, 0, 0, 0],
        ];

        let answer = [
            [4, 3, 5, 2, 6, 9, 7, 8, 1],
            [6, 8, 2, 5, 7, 1, 4, 9, 3],
            [1, 9, 7, 8, 3, 4, 5, 6, 2],
            [8, 2, 6, 1, 9, 5, 3, 4, 7],
            [3, 7, 4, 6, 8, 2, 9, 1, 5],
            [9, 5, 1, 7, 4, 3, 6, 2, 8],
            [5, 1, 9, 3, 2, 6, 8, 7, 4],
            [2, 4, 8, 9, 5, 7, 1, 3, 6],
            [7, 6, 3, 4, 1, 8, 2, 5, 9],
        ];

        assert_eq!(crate::solve(puzzle).unwrap(), answer);

        let puzzle = [
            [0, 0, 0, 6, 0, 0, 4, 0, 0],
            [7, 0, 0, 0, 0, 3, 6, 0, 0],
            [0, 0, 0, 0, 9, 1, 0, 8, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 5, 0, 1, 8, 0, 0, 0, 3],
            [0, 0, 0, 3, 0, 6, 0, 4, 5],
            [0, 4, 0, 2, 0, 0, 0, 6, 0],
            [9, 0, 3, 0, 0, 0, 0, 0, 0],
            [0, 2, 0, 0, 0, 0, 1, 0, 0],
        ];

        let answer = [
            [5, 8, 1, 6, 7, 2, 4, 3, 9],
            [7, 9, 2, 8, 4, 3, 6, 5, 1],
            [3, 6, 4, 5, 9, 1, 7, 8, 2],
            [4, 3, 8, 9, 5, 7, 2, 1, 6],
            [2, 5, 6, 1, 8, 4, 9, 7, 3],
            [1, 7, 9, 3, 2, 6, 8, 4, 5],
            [8, 4, 5, 2, 1, 9, 3, 6, 7],
            [9, 1, 3, 7, 6, 8, 5, 2, 4],
            [6, 2, 7, 4, 3, 5, 1, 9, 8],
        ];

        assert_eq!(crate::solve(puzzle).unwrap(), answer);
    }
}

    // Example sudoku puzzles

    // LA Times
    // let puzzle = [
    //     [9, 0, 0, 0, 3, 0, 0, 0, 2],
    //     [0, 0, 0, 5, 4, 0, 0, 0, 0],
    //     [0, 2, 0, 7, 0, 0, 0, 0, 4],
    //     [0, 0, 0, 0, 0, 0, 6, 0, 5],
    //     [0, 6, 0, 2, 0, 5, 0, 0, 0],
    //     [7, 0, 1, 0, 0, 0, 0, 0, 0],
    //     [2, 0, 0, 0, 0, 9, 0, 8, 0],
    //     [0, 0, 0, 0, 5, 1, 3, 0, 0],
    //     [4, 0, 0, 0, 0, 0, 0, 0, 9],
    // ];
