extern crate groio;

use std::fs::File;
use std::io::Read;
use groio::*;

fn main() {
    let mut file = File::open("resources/dppc.gro").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let structure: Structure = s.parse().unwrap();

    println!("title => {}", structure.title());
    let atoms = structure.atoms();
    println!("num atoms => {}", atoms.len());
    println!("first atom => {}", atoms[0]);
    println!("second atom => {}", atoms[1]);
    println!("box size => {}", structure.box_size());
}
