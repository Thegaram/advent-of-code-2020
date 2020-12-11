use itertools::zip;
use std::io::{self, BufRead};

type Field = char;
type Grid = Vec<Vec<Field>>;

const FLOOR: Field = '.';
const EMPTY: Field = 'L';
const OCCUPIED: Field = '#';

fn read_grid() -> Grid {
    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn seat(grid: &Grid, row: i32, col: i32) -> Field {
    if row < 0 || row as usize >= grid.len() || col < 0 || col as usize >= grid[0].len() {
        FLOOR
    } else {
        grid[row as usize][col as usize]
    }
}

fn count_occupied_around(grid: &Grid, row: usize, col: usize) -> usize {
    let row = row as i32;
    let col = col as i32;

    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .into_iter()
    .map(|(row, col)| seat(&grid, row, col))
    .filter(|seat| *seat == OCCUPIED)
    .count()
}

fn count_occupied_seen(grid: &Grid, row: usize, col: usize) -> usize {
    let above = || (0..row).rev();
    let same_row = || std::iter::repeat(row);
    let below = || (row + 1)..grid.len();
    let left = || (0..col).rev();
    let same_col = || std::iter::repeat(col);
    let right = || (col + 1)..grid[0].len();
    let field = |(r, c): (usize, usize)| grid[r][c];
    let not_empty = |f: &Field| *f != FLOOR;

    let mut count = 0;

    // top
    if zip(above(), same_col()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // top-right
    if zip(above(), right()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // right
    if zip(same_row(), right()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // bottom-right
    if zip(below(), right()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // bottom
    if zip(below(), same_col()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // bottom-left
    if zip(below(), left()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // left
    if zip(same_row(), left()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    // top-left
    if zip(above(), left()).map(field).find(not_empty) == Some(OCCUPIED) {
        count += 1;
    }

    count
}

fn count_occupied(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|x| **x == OCCUPIED)
        .count()
}

fn step(
    grid: &Grid,
    threshold: usize,
    count_occupied: impl Fn(&Grid, usize, usize) -> usize,
) -> Grid {
    let mut new_grid = grid.clone();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let seat = seat(&grid, row as i32, col as i32);
            let occupied = count_occupied(&grid, row, col);

            match (seat, occupied) {
                (EMPTY, 0) => new_grid[row][col] = OCCUPIED,
                (OCCUPIED, n) if n >= threshold => new_grid[row][col] = EMPTY,
                _ => {}
            }
        }
    }

    new_grid
}

fn find_equilibrium(
    mut grid: Grid,
    threshold: usize,
    count_occupied: impl Fn(&Grid, usize, usize) -> usize,
) -> Grid {
    loop {
        match step(&grid, threshold, &count_occupied) {
            new if new == grid => break,
            new => grid = new,
        }
    }

    grid
}

fn main() {
    let grid = read_grid();

    let grid_1 = find_equilibrium(grid.clone(), 4, &count_occupied_around);
    println!("puzzle #1 = {}", count_occupied(&grid_1));

    let grid_2 = find_equilibrium(grid, 5, &count_occupied_seen);
    println!("puzzle #2 = {}", count_occupied(&grid_2));
}
