use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut f = File::open("rosalind_revc.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

fn swap_base(base: char) -> char {
    match base {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => { unreachable!() }
    }
}

fn complement_bases(bases: String) -> String {
    bases
      .trim()
      .chars()
      .rev()
      .map(swap_base)
      .collect::<String>()
}

fn main() {
    let bases = read_file();
    let complement = complement_bases(bases);
    println!("{}", complement);
}
