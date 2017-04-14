use std::fmt;
use std::collections::HashMap;
use biost::Vector3d;

#[derive(Clone)]
pub struct Atom {
    pub res_number:  i32,
    pub res_name:    String,
    pub atom_name:   String,
    pub atom_number: i32,
    pub position:    Vector3d,
    pub velocity:    Vector3d,
}

#[derive(Clone)]
pub struct Residue {
    pub number: i32,
    pub name: String,
    pub atoms: HashMap<String, Atom>,
}

pub struct Structure {
    title: String,
    atoms: Vec<Atom>,
    box_size: Vector3d,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5}{:<5}{:>5}{:>5}{:>8.3}{:>8.3}{:>8.3}{:>9.4}{:>9.4}{:>9.4}",
               self.res_number,
               self.res_name,
               self.atom_name,
               self.atom_number,
               self.position.x,
               self.position.y,
               self.position.z,
               self.velocity.x,
               self.velocity.y,
               self.velocity.z)
    }
}

impl Residue {
    pub fn new(number: i32, name: &str) -> Residue {
        Residue {
            number: number,
            name:   name.to_string(),
            atoms:  HashMap::new()
        }
    }
}

impl Structure {
    pub fn new(title: String, atoms: Vec<Atom>, box_size: Vector3d) -> Structure {
        Structure {
            title:    title,
            atoms:    atoms,
            box_size: box_size
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    pub fn residues(&self) -> Vec<Residue> {
        let max_size = self.atoms.iter()
                                 .map(|atom| atom.res_number)
                                 .max()
                                 .unwrap_or(0)
                                 as usize;
        let mut residues = vec![Residue::new(0, ""); max_size];
        for atom in &self.atoms {
            let idx = (atom.res_number - 1) as usize;
            if residues[idx].name.is_empty() {
                residues[idx].number = atom.res_number;
                residues[idx].name = atom.res_name.clone();
            } else {
                assert!(residues[idx].number == atom.res_number);
                assert!(residues[idx].name == atom.res_name);
            }
            residues[idx].atoms.insert(atom.atom_name.clone(), atom.clone());
        }
        residues
    }

    pub fn box_size(&self) -> &Vector3d {
        &self.box_size
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title)?;
        let atoms = self.atoms();
        writeln!(f, "{}", atoms.len())?;
        for atom in atoms {
            writeln!(f, "{}", atom)?;
        }
        write!(f, "{:>10.4}{:>10.4}{:>10.4}",
               self.box_size.x,
               self.box_size.y,
               self.box_size.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("    1ALA      H    1   1.000   2.000   3.000   4.0000   5.0000   6.0000",
                   atom.to_string());
    }

    #[test]
    fn test_structure_to_string() {
        let structure = Structure::new(
            "The Title".to_string(),
            Vec::new(),
            Vector3d::new(1.0, 2.0, 3.0));
        assert_eq!("The Title\n0\n    1.0000    2.0000    3.0000",
                   structure.to_string());
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

    #[test]
    fn test_strucutre_residues() {
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
        assert_eq!(1, structure.residues().len());
    }
}

