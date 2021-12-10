use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

// NOTE: In order to correctly run, you need to determine a lenght of the bits
// and set it in the BITLEN.
const BITLEN: usize = 12;
// const BITLEN: usize = 5;
const EPSMASK: i64 = (1 << BITLEN) - 1;

/// Creates an iterator over the input file `path`
fn read_input(path: &Path) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

#[inline]
fn epsilon_rate(gamma_rate: i64) -> i64 {
    !gamma_rate & EPSMASK
}

#[inline]
fn gamma_rate(gamma_bits: &[u32; BITLEN], total_size: i64) -> i64 {
    let mut gamma: i64 = 0;
    for (pos, bit) in gamma_bits.iter().enumerate() {
        gamma |= (((total_size as i64 - *bit as i64) >> 63) & 0x1) << pos;
    }
    gamma
}

fn oxygen_rate(it: &Vec<u32>, offset: usize) -> u32 {
    let most_common = find_most_common(&it, offset);

    let tmp_it = it
        .iter()
        .copied()
        .filter(|e| (*e >> offset) & 0x1 == most_common)
        .collect::<Vec<u32>>();
    if offset == 0 || tmp_it.len() < 2 {
        return tmp_it[0];
    }
    oxygen_rate(&tmp_it, offset - 1)
}

fn co2_rate(it: &Vec<u32>, offset: usize) -> u32 {
    let most_common = find_most_common(&it, offset);

    let tmp_it = it
        .iter()
        .copied()
        .filter(|e| {
            let x = e >> offset;
            x & 0x1 != most_common
        })
        .collect::<Vec<u32>>();
    if offset == 0 || tmp_it.len() < 2 {
        return tmp_it[0];
    }
    co2_rate(&tmp_it, offset - 1)
}

fn find_most_common(collection: &Vec<u32>, offset: usize) -> u32 {
    let mut num = 0;
    collection.into_iter().for_each(|el| {
        num += el >> offset & 0x1;
    });
    if collection.len() as u32 - num <= num {
        1
    } else {
        0
    }
}

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
    let path = Path::new("../input.txt");
    let (bit_freq, total_size) = read_report(read_input(path).unwrap());
    let gamma = gamma_rate(&bit_freq, total_size);
    let epsilon = epsilon_rate(gamma);
    println!("Gamma = {}\nEpsilon = {}", gamma, epsilon);
    println!("Total consumption = {}", gamma * epsilon);

    let input = read_input(path)
        .unwrap()
        .map(|e| u32::from_str_radix(&e.unwrap(), 2).unwrap())
        .collect::<Vec<u32>>();

    let oxrate = oxygen_rate(&input, BITLEN - 1);
    let corate = co2_rate(&input, BITLEN - 1);
    println!("Oxygen = {}, CO2 = {}", oxrate, corate);
    println!("Total emmision is {}", oxrate * corate);
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn simple_gamma() {
        let (bit_freq, total_size) = read_report(read_input(Path::new("test.txt")).unwrap());
        let gamma = gamma_rate(&bit_freq, total_size);
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon_rate(gamma)); //espilon
    }

    #[test]
    fn oxygen_test() {
        let path = Path::new("test.txt");
        let input = read_input(path)
            .unwrap()
            .map(|e| u32::from_str_radix(&e.unwrap(), 2).unwrap())
            .collect::<Vec<u32>>();

        let corate = co2_rate(&input, BITLEN - 1);
        let oxrate = oxygen_rate(&input, BITLEN - 1);
        assert_eq!(23, oxrate);
        assert_eq!(10, corate);
    }
}
