use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

const BROW_SZ: usize = 5;
const BCOL_SZ: usize = 5;

#[derive(Debug)]
struct Board {
    _inner: [u32; BROW_SZ * BCOL_SZ],
    populus: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "|")?;
        for (pos, val) in self._inner.iter().enumerate() {
            if pos % BROW_SZ == 0 && pos != 0 {
                write!(f, "|\n|")?;
            }
            write!(f, "{:3}", val)?;
        }
        write!(f, "|")
    }
}

impl Board {
    fn new() -> Self {
        Board {
            _inner: [0; BROW_SZ * BCOL_SZ],
            populus: 0,
        }
    }

    fn populate_row(&mut self, values: impl Iterator<Item = u32>) {
        for val in values {
            self._inner[self.populus] = val;
            self.populus += 1;
        }
    }

    // #[inline]
    // fn at(&self, row: usize, col: usize) -> u32 {
    //     self._inner[row * 5 + col]
    // }

    #[inline]
    /// Checks if the board has filled any row or col
    fn hit_bingo(&self, pos: usize) -> bool {
        // check for bingo
        let row = pos / BROW_SZ;
        let col = pos - row * BCOL_SZ;

        // the board has cleared the row
        if self._inner[row * BROW_SZ..row * BROW_SZ + 5]
            .iter()
            .sum::<u32>()
            == 0
        {
            // println!("Winning row: {} col: {}", row, col);
            return true;
        }

        // the board has cleared the column
        if self._inner[col..].iter().step_by(BCOL_SZ).sum::<u32>() == 0 {
            // println!("Winning col: {} row: {}", col, row);
            return true;
        }

        false
    }

    /// if the board contains drawn number, it sets it to 0
    fn bingo(&mut self, draw: u32) -> bool {
        for (pos, num) in self._inner.iter_mut().enumerate() {
            if *num == draw {
                *num = 0;
                return self.hit_bingo(pos);
            };
        }
        false
    }

    /// Returns the sum of the unmarked board values
    fn get_sum(&self) -> u32 {
        self._inner.iter().sum()
    }
}

#[derive(Debug)]
struct Game {
    _numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn from_lines(mut lines: Lines<BufReader<File>>) -> Self {
        let winners = lines
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|e| e.parse::<u32>().unwrap())
            .collect();
        let mut boards = Vec::new();
        let mut board = Board::new();
        for line in lines {
            let line = line.unwrap();
            if line == "\n" {
                continue;
            }
            if board.populus == BROW_SZ * BCOL_SZ {
                boards.push(board);
                board = Board::new();
            }
            board.populate_row(line.split_whitespace().map(|e| e.parse::<u32>().unwrap()));
        }
        boards.push(board);
        Game {
            _numbers: winners,
            boards,
        }
    }

    /// Plays a game of bingo
    fn play_first_winner(&mut self) -> u32 {
        for draw in self._numbers.clone() {
            let result = self.check_boards(draw);
            if result != 0 {
                return result;
            }
        }
        0
    }

    // messy solution.
    fn play_last_winner(&mut self) -> u32 {
        let mut last_result = 0;
        let mut finished_boards = VecDeque::new();
        for draw in self._numbers.clone() {
            for (pos, board) in self.boards.iter_mut().enumerate() {
                if board.bingo(draw) {
                    last_result = Game::finish(board, draw);
                    finished_boards.push_back(pos);
                }
            }
            while let Some(pos) = finished_boards.pop_back() {
                self.boards.remove(pos);
            }
        }
        last_result
    }

    #[inline(always)]
    fn finish(board: &Board, drawn_number: u32) -> u32 {
        // println!("Final number: {}\nSum: {}", drawn_number, board.get_sum());
        board.get_sum() * drawn_number
    }

    fn check_boards(&mut self, draw: u32) -> u32 {
        for board in self.boards.iter_mut() {
            if board.bingo(draw) {
                println!("Winning board: \n{}", board);
                return Game::finish(board, draw);
            }
        }
        0
    }
}

/// Creates an iterator over the input file `path`
fn read_input(path: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn main() {
    let input = read_input(Path::new("../input.txt")).unwrap();
    let mut game = Game::from_lines(input);
    assert_eq!(100, game.boards.len());
    let result = game.play_first_winner();
    println!("Final score is: {}", result);

    let result = game.play_last_winner();
    println!("Last winning result is: {}", result);
    // for brd in game.boards {
    //     println!("{}\n", brd);
    // }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn simple_test() {
        let lines = read_input(Path::new("test.txt")).unwrap();
        let mut game = Game::from_lines(lines);

        assert_ne!(0, game.boards.len());
        let result = game.play_first_winner();
        assert_eq!(4512, result);
    }

    #[test]
    fn last_result() {
        let lines = read_input(Path::new("test.txt")).unwrap();
        let mut game = Game::from_lines(lines);

        assert_ne!(0, game.boards.len());
        let result = game.play_last_winner();
        assert_eq!(1924, result);
    }
}
