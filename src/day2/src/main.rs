use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

impl Position {
    fn new() -> Position {
        Position {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    #[inline]
    fn parse_commands(&mut self, it: Lines<BufReader<File>>) {
        for line in it {
            let l = line.unwrap();
            self.eat_command(&l);
        }
    }

    fn eat_command(&mut self, command: &String) {
        if let Some((com, num)) = command.split_once(" ") {
            let num = num.parse::<u32>().unwrap();
            match com {
                "forward" => {
                    self.horizontal += num;
                }
                "down" => self.depth += num,
                "up" => self.depth -= num,
                _ => unreachable!("Logical invariant"),
            }
        }
    }

    #[inline]
    fn parse_commands2(&mut self, it: Lines<BufReader<File>>) {
        for line in it {
            let l = line.unwrap();
            self.eat_command2(&l);
        }
    }

    fn eat_command2(&mut self, command: &String) {
        if let Some((com, num)) = command.split_once(" ") {
            let num = num.parse::<u32>().unwrap();
            match com {
                "forward" => {
                    self.horizontal += num;
                    self.depth += num * self.aim;
                }
                "down" => self.aim += num,
                "up" => self.aim -= num,
                _ => unreachable!("Logical invariant"),
            }
        }
    }

    #[inline]
    fn total_distance(&self) -> u32 {
        self.depth * self.horizontal
    }

    #[inline]
    fn refresh(&mut self) {
        self.depth = 0;
        self.aim = 0;
        self.horizontal = 0;
    }
}

/// Creates an iterator over the input file `path`
fn commands(path: &Path) -> io::Result<Lines<BufReader<File>>> {
    println!("Reading file: {:?}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn main() {
    println!("Hello, #AOC2021 day2!");
    let mut pos = Position::new();
    pos.parse_commands(commands(Path::new("../input.txt")).unwrap());
    println!("Covered distance = {}", pos.total_distance());

    pos.refresh();
    pos.parse_commands2(commands(Path::new("../input.txt")).unwrap());
    println!("Covered distance with aim = {}", pos.total_distance());
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn simple_test() {
        let mut pos = Position::new();
        pos.parse_commands(commands(Path::new("test.txt")).unwrap());
        assert_eq!(40, pos.total_distance());
    }

    #[test]
    fn simple_test2() {
        let mut pos = Position::new();
        pos.parse_commands2(commands(Path::new("test.txt")).unwrap());
        assert_eq!(180, pos.total_distance());
    }
}
