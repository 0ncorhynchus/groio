use std::str::FromStr;
use error::*;

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

impl FromStr for Atom {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Atom {
            res_number:  s[0..5].parse()?,
            res_name:    s[5..10].to_string(),
            atom_name:   s[10..15].to_string(),
            atom_number: s[15..20].parse()?,
            position:    Vector3d::new(s[20..28].parse()?,
                                       s[28..36].parse()?,
                                       s[36..44].parse()?),
            velocity:    Vector3d::new(s[44..52].parse()?,
                                       s[52..60].parse()?,
                                       s[60..68].parse()?),
        })
    }
}

pub struct Structure {
    pub title: String,
    pub box_size: Vector3d,
    pub atoms: Vec<Atom>,
}
