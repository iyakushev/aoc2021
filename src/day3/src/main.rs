use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

// NOTE: In order to correctly run, you need to determine a lenght of the bits
// and set it in the BITLEN.
const BITLEN: usize = 12;
const EPSMASK: i64 = (1 << BITLEN) - 1;

/// Creates an iterator over the input file `path`
fn read_input(path: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn epsilon_rate(gamma_rate: i64) -> i64 {
    !gamma_rate & EPSMASK
}

fn gamma_rate(gamma_bits: &[u32; BITLEN], total_size: i64) -> i64 {
    let mut gamma: i64 = 0;
    for (pos, bit) in gamma_bits.iter().enumerate() {
        gamma |= (((total_size as i64 - *bit as i64) >> 63) & 0x1) << pos;
    }
    gamma
}

// fn oxygen_rate(bit_freq: &[u32; BITLEN], total_size: i64) -> i64 {}

fn read_report(it: Lines<BufReader<File>>) -> ([u32; BITLEN], i64) {
    let mut gamma_bits: [u32; BITLEN] = [0; BITLEN];
    let mut total_size: i64 = 0;
    let mut num: u32;
    for numeric in it {
        num = u32::from_str_radix(&numeric.unwrap(), 2).unwrap();
        // should be unrolled. I hope so, at least
        gamma_bits.iter_mut().enumerate().for_each(|(pos, value)| {
            *value += (num >> pos) & 0x1;
        });
        total_size += 1;
    }
    // recalculate threshold for most common bit
    (gamma_bits, total_size / 2)
}

fn main() {
    println!("Hello, #AOC2021 day3!");
    let (bit_freq, total_size) = read_report(read_input(Path::new("../input.txt")).unwrap());
    let gamma = gamma_rate(&bit_freq, total_size);
    let epsilon = epsilon_rate(gamma);
    println!("Gamma = {}\nEpsilon = {}", gamma, epsilon);
    println!("Total consumption = {}", gamma * epsilon);
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn simple_gamma() {
        let (bit_freq, total_size) = read_report(read_input(Path::new("../test.txt")).unwrap());
        let gamma = gamma_rate(&bit_freq, total_size);
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon_rate(gamma)); //espilon
    }

    #[test]
    fn oxygen_test() {
        let (bit_freq, total_size) = read_report(read_input(Path::new("../test.txt")).unwrap());
    }
}
