use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file() -> String {
    let mut f = File::open("rosalind_dna.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

fn count_bases(bases: String) -> (u32, u32, u32, u32) {

    let mut acc = HashMap::new();

    for base in bases.trim().chars() {
        let counter = acc.entry(base).or_insert(0 as u32);
        *counter += 1;
    }

    (acc[&'A'], acc[&'C'], acc[&'G'], acc[&'T'])
}

fn main() {
    let bases = read_file();
    let (a, c, g, t) = count_bases(bases);
    println!("{:?} {:?} {:?} {:?}", a, c, g, t);
}

