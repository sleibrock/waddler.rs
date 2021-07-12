// structs/sidedef.rs

use utils::{u8_to_u16, u8_to_i16, u8_to_string};
use structs::constants::SIDEDEF_W;

pub struct SideDef {
    pub x_offset:   i16,
    pub y_offset:   i16,
    pub sector:     u16,
    pub upper_tex:  String,
    pub lower_tex:  String,
    pub middle_tex: String,
}


impl SideDef {
    pub fn new(dat: &[u8]) -> Result<SideDef, String> {
	match dat.len() {
	    SIDEDEF_W => {
		let mut zero1 : usize = 11;
		let mut zero2 : usize = 19;
		let mut zero3 : usize = 27;
		while dat[zero1] == 0 { zero1 -= 1; }
		while dat[zero2] == 0 { zero2 -= 1; }
		while dat[zero3] == 0 { zero3 -= 1; }
		
		Ok(SideDef {
		    x_offset:   u8_to_i16(&dat[0..1]),
		    y_offset:   u8_to_i16(&dat[2..3]),
		    sector:     u8_to_u16(&dat[28..29]),
		    upper_tex:  u8_to_string(&dat[4..zero1+1]),
		    lower_tex:  u8_to_string(&dat[12..zero2+1]),
		    middle_tex: u8_to_string(&dat[20..zero3+1]),
		})
	    },
	    bytes => {
		Err(format!("Invalid bytes length (given {})", bytes))
	    }
	}
    }
}


// end structs/sidedef.rs
