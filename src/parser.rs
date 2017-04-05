use std::str::{FromStr, Lines};
use error::*;
use types::*;

impl FromStr for Atom {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Atom {
            res_number:  s[0..5].trim().parse()?,
            res_name:    s[5..10].trim().to_string(),
            atom_name:   s[10..15].trim().to_string(),
            atom_number: s[15..20].trim().parse()?,
            position:    Vector3d::new(s[20..28].trim().parse()?,
                                       s[28..36].trim().parse()?,
                                       s[36..44].trim().parse()?),
            velocity:    Vector3d::new(s[44..52].trim().parse()?,
                                       s[52..60].trim().parse()?,
                                       s[60..68].trim().parse()?),
        })
    }
}

fn next_line<'a>(lines: &mut Lines<'a>) -> Result<&'a str> {
    lines.next().ok_or(Error::LackOfLine)
}

fn parse_box_size(s: &str) -> Result<Vector3d> {
    let splitted: Vec<_> = s.split_whitespace().collect();
    if splitted.len() == 3 {
        Ok(Vector3d::new(splitted[0].trim().parse()?,
                         splitted[1].trim().parse()?,
                         splitted[2].trim().parse()?))
    } else {
        Err(Error::Parse(ParseError::InvalidStatement))
    }
}

impl FromStr for Structure {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let title = next_line(&mut lines)?.to_string();
        let num_atoms: usize = next_line(&mut lines)?.trim().parse()?;
        let mut atoms = Vec::new();
        for _ in 0..num_atoms {
            atoms.push(next_line(&mut lines)?.parse()?);
        }
        Ok(Structure::new(title, atoms, parse_box_size(next_line(&mut lines)?)?))
    }
}
