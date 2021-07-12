// structs/lump.rs

use std::fmt;
use utils::*;
use structs::constants::*;

pub struct Lump {
    pub name:     String,
    pub posn:     usize,
    pub size:     usize,
    pub start:    usize,
    pub end:      usize,
    pub is_level: bool,
}


impl Lump {
    pub fn new(dat: &[u8]) -> Result<Lump, String> {
	match dat.len() {
	    LUMP_W => {
		// find the first non-null byte from the right side
		// very weird one-line iteration trick, yes
		let first_zero = find_zero_from_right(&dat, 15);
		//let mut first_zero : usize = 15;
		//while dat[first_zero] == 0 { first_zero -= 1; }
		
		// check if the lump name matches:
		// NAME = ExMx or NAME = MAPxx
		let mut is_level_lump = false;
		if (dat[8]==69&&dat[10]==77)
		    || (dat[8]==77&&dat[9]==65&&dat[10]==80){
			if first_zero == 11 || first_zero == 12 {
			    is_level_lump = true;
			}
		    }
		
		// these are two-byte slice conversions
		// used to calculate position and size
		let p = u8_to_usize(&dat[0..3]);
		let s = u8_to_usize(&dat[4..7]);
		
		let p_h = match is_level_lump {
		    false => p - HEADER_W,
		    _     => 0,
		};
		
		Ok(Lump {
		    is_level: is_level_lump,
		    posn:     p,
		    size:     s,
		    start:    p_h,
		    end:      p_h + s,
		    name:     u8_to_string(&dat[8..(first_zero+1)]),
		})
	    },

	    // If invalid bytes, error out
	    bytes => Err(format!("Invalid amount of bytes (given {})", bytes)),
	}
    }
}



// debug printer for a Lump
impl fmt::Debug for Lump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <0x{:X} .. 0x{:X}>",
	       self.name, self.posn,
	       self.posn+self.size)
    }
}


// end structs/lump.rs
