use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut f = File::open("rosalind_rna.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}


fn convert_dna_to_rna(dna: String) -> String {
    dna.replace("T", "U")
}

fn main() {
    let dna = read_file();
    let rna = convert_dna_to_rna(dna);
    println!("{}", rna);
}
