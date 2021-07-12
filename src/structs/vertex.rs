// structs/vertex.rs

use utils::u8_to_i16;
use structs::constants::VERTEX_W;


pub struct Vertex {
    pub x: i16,
    pub y: i16,
}


impl Vertex {
    pub fn new(dat: &[u8]) -> Result<Vertex, String> {
	match dat.len() {
	    VERTEX_W => Ok(Vertex {
		x: u8_to_i16(&dat[0..1]),
		y: u8_to_i16(&dat[2..3]),
	    }),

	    b => Err(format!("Invalid bytes (given {})", b)),
	}
    }
}


// end structs/vertex.rs
