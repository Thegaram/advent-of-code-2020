use std::io::{self, BufRead};

const TREE: char = '#';

#[derive(Debug)]
struct Map {
    raw: Vec<Vec<char>>,
    pub rows: usize,
    pub columns: usize,
}

impl From<Vec<Vec<char>>> for Map {
    fn from(raw: Vec<Vec<char>>) -> Map {
        let rows = raw.len();
        let columns = raw[0].len();
        Map { raw, rows, columns }
    }
}

impl Map {
    fn at(&self, row: usize, col: usize) -> char {
        self.raw[row][col]
    }
}

fn count_trees(map: &Map, slope: (usize, usize)) -> usize {
    let mut count = 0;
    let mut row = 0;
    let mut col = 0;

    loop {
        if row >= map.rows {
            return count;
        }

        if map.at(row, col % map.columns) == TREE {
            count += 1;
        }

        row += slope.0;
        col += slope.1;
    }
}

fn puzzle_1(map: &Map) -> usize {
    count_trees(map, (1, 3))
}

fn puzzle_2(map: &Map) -> usize {
    count_trees(map, (1, 1))
        * count_trees(map, (1, 3))
        * count_trees(map, (1, 5))
        * count_trees(map, (1, 7))
        * count_trees(map, (2, 1))
}

fn main() {
    let stdin = io::stdin();

    let map: Map = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();

    println!("puzzle #1: {:?}", puzzle_1(&map));
    println!("puzzle #2: {:?}", puzzle_2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_trees() {
        let map = Map::from(vec![
            vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '#'],
            vec!['.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '#'],
        ]);

        assert_eq!(count_trees(&map, (1, 1)), 2);
        assert_eq!(count_trees(&map, (1, 3)), 7);
        assert_eq!(count_trees(&map, (1, 5)), 3);
        assert_eq!(count_trees(&map, (1, 7)), 4);
        assert_eq!(count_trees(&map, (2, 1)), 2);
    }
}
