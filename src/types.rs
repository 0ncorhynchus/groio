use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone)]
pub struct Atom {
    pub res_number:  i32,
    pub res_name:    String,
    pub atom_name:   String,
    pub atom_number: i32,
    pub position:    Vector3d,
    pub velocity:    Vector3d,
}

pub type Residue = HashMap<String, Atom>;

pub struct Structure {
    title: String,
    residues: Vec<Residue>,
    box_size: Vector3d,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3d { x: x, y: y, z: z }
    }
}

impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prec = f.precision().unwrap_or(4);
        let width = f.width().unwrap_or(10);
        write!(f, "{:width$.prec$}{:width$.prec$}{:width$.prec$}",
               self.x, self.y, self.z, width=width, prec=prec)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5}{:<5}{:>5}{:>5}{:>8.3}{:>9.4}",
               self.res_number,
               self.res_name,
               self.atom_name,
               self.atom_number,
               self.position,
               self.velocity)
    }
}

fn split_to_residue(atoms: Vec<Atom>) -> Vec<Residue> {
    let max_size = atoms.iter().map(|atom| atom.res_number).max().unwrap_or(-1) + 1;
    let mut residues = vec![Residue::new(); max_size as usize];
    for atom in atoms {
        residues[atom.res_number as usize].insert(atom.atom_name.clone(), atom);
    }
    residues
}

impl Structure {
    pub fn new(title: String, atoms: Vec<Atom>, box_size: Vector3d) -> Structure {
        Structure {
            title: title,
            residues: split_to_residue(atoms),
            box_size: box_size
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn atoms(&self) -> Vec<&Atom> {
        let mut atoms = Vec::new();
        for i in 0..self.residues.len() {
            let residue = &self.residues[i];
            for key in residue.keys() {
                match residue.get(key) {
                    Some(atom) => atoms.push(atom),
                    None => {}
                }
            }
        }
        atoms.sort_by_key(|&atom| atom.atom_number);
        atoms
    }

    pub fn box_size(&self) -> &Vector3d {
        &self.box_size
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)?;
        for atom in &self.atoms() {
            writeln!(f, "{}", atom)?;
        }
        write!(f, "{}", self.box_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_to_string() {
        let vector = Vector3d::new(1.0, 2.0, 3.0);
        assert_eq!("    1.0000    2.0000    3.0000",
                   format!("{}", vector));
    }

    #[test]
    fn test_atom_to_string() {
        let atom = Atom {
            res_number: 1,
            res_name: "ALA".to_string(),
            atom_name: "H".to_string(),
            atom_number: 1,
            position: Vector3d::new(1.0, 2.0, 3.0),
            velocity: Vector3d::new(4.0, 5.0, 6.0)
        };
        assert_eq!("    1ALA      H    1   1.000   2.000   3.000   4.0000   5.0000   6.0000", atom.to_string());
    }

    #[test]
    fn test_structure_to_string() {
        let structure = Structure::new(
            "The Title".to_string(),
            Vec::new(),
            Vector3d::new(1.0, 2.0, 3.0));
        assert_eq!("The Title\n    1.0000    2.0000    3.0000", structure.to_string());
    }

    #[test]
    fn test_structure_new() {
        Structure::new(
            "The Title".to_string(),
            vec![
                Atom {
                    res_number: 1,
                    res_name: "ALA".to_string(),
                    atom_name: "H".to_string(),
                    atom_number: 1,
                    position: Vector3d::new(1.0, 2.0, 3.0),
                    velocity: Vector3d::new(4.0, 5.0, 6.0)
                }
            ],
            Vector3d::new(11.0, 12.0, 13.0)
        );
    }

    #[test]
    fn test_structure_atoms() {
        let structure = Structure::new(
            "The Title".to_string(),
            vec![
                Atom {
                    res_number: 1,
                    res_name: "ALA".to_string(),
                    atom_name: "H".to_string(),
                    atom_number: 1,
                    position: Vector3d::new(1.0, 2.0, 3.0),
                    velocity: Vector3d::new(4.0, 5.0, 6.0)
                }
            ],
            Vector3d::new(11.0, 12.0, 13.0)
        );
        assert_eq!(1, structure.atoms().len());
    }
}

