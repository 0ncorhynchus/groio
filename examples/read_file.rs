extern crate groio;

use std::fs::File;
use std::io::Read;
use groio::*;

fn main() {
    let mut file = File::open("resources/dppc.gro").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let structure: Structure = s.parse().unwrap();

    println!("title => {}", structure.title);
    println!("num atoms => {}", structure.atoms.len());
    println!("first atom => {}", structure.atoms[0]);
    println!("second atom => {}", structure.atoms[1]);
    println!("box size => {}", structure.box_size);
}
