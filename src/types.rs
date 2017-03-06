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

pub struct Atom {
    pub res_number:  i32,
    pub res_name:    String,
    pub atom_number: i32,
    pub atom_name:   String,
    pub position:    Vector3d,
    pub velocity:    Vector3d,
}

pub struct Structure {
    pub title: String,
    pub atoms: Vec<Atom>,
    pub box_size: Vector3d,
}

