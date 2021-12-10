use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

const BROW_SZ: usize = 5;
const BCOL_SZ: usize = 5;

#[derive(Debug)]
struct Board {
    _inner: [u32; BROW_SZ * BCOL_SZ],
    population_pos: usize,
}

impl Board {
    fn new() -> Self {
        Board {
            _inner: [0; BROW_SZ * BCOL_SZ],
            population_pos: 0,
        }
    }

    fn populate_row(&mut self, values: impl Iterator<Item = u32>) {
        for val in values {
            self._inner[self.population_pos * BROW_SZ] = val;
            self.population_pos += 1;
        }
    }

    #[inline]
    fn at(&self, row: usize, col: usize) -> u32 {
        self._inner[row * 5 + col]
    }

    /// if the board contains drawn number, it set to 0
    fn mark_on_board(&mut self, draw: u32) {
        for num in self._inner.iter_mut() {
            if *num == draw {
                *num = 0;
                break;
            };
        }
    }

    /// Returns the sum of the unmarked board values
    fn get_sum(&self) -> u32 {
        self._inner.iter().sum()
    }
}

#[derive(Debug)]
struct Game {
    _winning: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn from_lines(lines: &mut Lines<BufReader<File>>) -> Self {
        let winners = lines
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|e| e.parse::<u32>().unwrap())
            .collect();
        let mut boards = Vec::new();
        let mut board = Board::new();
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            println!("{}", line);
            if line == "\n" {
                continue;
            }
            println!("{}", board.population_pos);
            if board.population_pos == BROW_SZ * BCOL_SZ - 1 {
                boards.push(board);
                board = Board::new();
            }
            board.populate_row(line.split_whitespace().map(|e| e.parse::<u32>().unwrap()));
        }
        Game {
            _winning: winners,
            boards,
        }
    }
}

/// Creates an iterator over the input file `path`
fn read_input(path: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn main() {
    let mut input = read_input(Path::new("input.txt")).unwrap();
    let game = Game::from_lines(&mut input);
    println!("{:#?}", game);
}

#[cfg(test)]
pub mod test {

    use super::*;

    fn simple_test() {}
}
