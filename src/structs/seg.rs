// structs/seg.rs

use structs::constants::SEG_W;

/// Seg struct (segment)
/// TODO: this entire section is incomplete


pub struct Seg {
    pub start:     u16,
    pub end:       u16,
    pub angle:     i16,
    pub line_id:   u16,
    pub direction: u16,
    pub offset:    u16,
}


impl Seg {
    pub fn new(dat: &[u8]) -> Result<Seg, String> {
	match dat.len() {
	    SEG_W => {
		Ok(Seg {
		    start: 0,
		    end: 0,
		    angle: 0,
		    line_id: 0,
		    direction: 0,
		    offset: 0,
		})
	    },
	    bytes => {
		Err(format!("Invalid bytes length (given {})", bytes).into())
	    }
	}
    }
}
	

// end structs/seg.rs
