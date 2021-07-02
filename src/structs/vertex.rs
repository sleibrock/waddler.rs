// structs/vertex.rs

use utils::u8_to_i16;
use structs::constants::VERTEX_W;


pub struct Vertex {
    pub x: i16,
    pub y: i16,
}


impl Vertex {
    pub fn new(dat: &[u8]) -> Result<Vertex, String> {
        if dat.len() != VERTEX_W {
            return Err(format!("Vertex not given {} bytes", VERTEX_W).into());
        }

        Ok(Vertex {
            x: u8_to_i16(dat[0], dat[1]),
            y: u8_to_i16(dat[2], dat[3]),
        })
    }
}


// end structs/vertex.rs
