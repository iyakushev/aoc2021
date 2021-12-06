use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

/// Creates an iterator over the input file `path`
fn depth_iter(path: &Path) -> io::Result<Lines<BufReader<File>>> {
    println!("Reading file: {:?}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

/// Consumes iterator and calculates total inner depth increase
fn scan_depth(mut it: Lines<BufReader<File>>) -> u32 {
    let mut total_incr = 0u32;
    // I'm on a train AND I'm lazy, so .unwrap().unwrap().unwrap()
    let mut value = it.next().unwrap().unwrap().parse::<i32>().unwrap(); // integer to allow negative results
    for read in it {
        if let Ok(num) = read {
            let tmp = num.parse::<i32>().unwrap();
            // simply add the highest bit of the difference
            total_incr += (((value - tmp) as u32) >> 31) & 0x1;
            // println!("{} {} {} ", value, tmp, total_incr);
            value = tmp;
        }
    }
    total_incr
}

fn scan_depth_window(mut it: Lines<BufReader<File>>, window_size: usize) -> u32 {
    let it = it
        .map(|read| read.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut total_incr = 0u32;
    let mut lines = it.windows(window_size);
    let mut value = lines.next().unwrap().iter().sum::<i32>();
    for tup_values in lines {
        let tmp = tup_values.iter().sum();
        total_incr += (((value - tmp) as u32) >> 31) & 0x1;
        value = tmp
    }
    total_incr
}

fn main() {
    println!("Hello, #AOC2021 Day1");
    let result = scan_depth(depth_iter(Path::new("../input.txt")).unwrap());
    println!("Total increases: {}", result);

    let result = scan_depth_window(depth_iter(Path::new("../input.txt")).unwrap(), 3);
    println!("Total increases with window<3>: {}", result);
}

#[cfg(test)]
pub mod test {
    use std::path::Path;

    use crate::{depth_iter, scan_depth};

    #[test]
    fn simple_inc() {
        let depth_iterator = depth_iter(Path::new("test.txt")).unwrap();
        assert_eq!(5, scan_depth(depth_iterator))
    }
}
