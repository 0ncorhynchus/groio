use std::fmt;

pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3d { x: x, y: y, z: z }
    }
}

impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub struct Atom {
    pub res_number:  i32,
    pub res_name:    String,
    pub atom_name:   String,
    pub atom_number: i32,
    pub position:    Vector3d,
    pub velocity:    Vector3d,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {}",
               self.res_number,
               self.res_name,
               self.atom_name,
               self.atom_number,
               self.position,
               self.velocity,)
    }
}

pub struct Structure {
    pub title: String,
    pub atoms: Vec<Atom>,
    pub box_size: Vector3d,
}

