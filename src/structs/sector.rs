// structs/sector.rs

//use std::fmt;
use utils::*;
use structs::constants::*;

/// Sector struct
/// TODO: this entire section needs to be looked at

pub struct Sector {
    pub ceil:      u16,
    pub stag:      u16,
    pub floor:     u16,
    pub light:     u16,
    pub stype:     u16,
    pub ceil_tex:  String,
    pub floor_tex: String,
}


impl Sector {
    pub fn new(dat: &[u8]) -> Result<Sector, String> {
	match dat.len() {
	    SECTOR_W => {
		// Finding zeroes for the Sector lump
		//let mut zero1 : usize = 11;
		//let mut zero2 : usize = 19;
		let zero1 = find_zero_from_right(&dat, 11);
		let zero2 = find_zero_from_right(&dat, 19);
		//while dat[zero1] == 0 { zero1 -= 1; }
		//while dat[zero2] == 0 { zero2 -= 1; }
		
		Ok(Sector {
		    floor: u8_to_u16(&dat[0..2]),
		    ceil:  u8_to_u16(&dat[2..4]),
		    light: u8_to_u16(&dat[20..22]),
		    stype: u8_to_u16(&dat[2..4]),
		    stag:  u8_to_u16(&dat[2..4]),
		    floor_tex: u8_to_string(&dat[4..(zero1+1)]),
		    ceil_tex: u8_to_string(&dat[12..(zero2+1)]),
		})
	    },
	    bytes => {
		Err(format!("Invalid num of bytes given ({})", bytes).into())
	    }
	}
    }
}

// end structs/sector.rs
