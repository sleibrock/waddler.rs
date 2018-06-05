// structs/lump.rs

use std::fmt;
use std::ops::Range;
use utils::{u8_to_u32, u8_to_string};
use structs::constants::{HEADER_W, LUMP_W};

pub struct Lump {
    pub posn:     usize,
    pub size:     usize,
    pub name:     String,
    pub is_level: bool,
}


impl Lump {
    pub fn new(dat: &[u8]) -> Lump {
        if dat.len() != LUMP_W {
            panic!(format!("Lump not given {} bytes", LUMP_W));
        }

        // find the first non-null byte from the right side
        let mut first_zero : usize = 15;
        while dat[first_zero] == 0 { first_zero -= 1; }
        
        // check if the lump name matches:
        // NAME = ExMx or NAME = MAPxx
        let mut is_level_lump = false;
        if (dat[8]==69&&dat[10]==77)||(dat[8]==77&&dat[9]==65&&dat[10]==80){
            if first_zero == 11 || first_zero == 12 {
                is_level_lump = true;
            }
        }

        Lump {
            is_level: is_level_lump,
            posn:     u8_to_u32(dat[0], dat[1], dat[2], dat[3]) as usize,
            size:     u8_to_u32(dat[4], dat[5], dat[6], dat[7]) as usize,
            name:     u8_to_string(&dat[8..(first_zero+1)]),
        }
    }

    pub fn range(&self) -> Range<usize> {
        ((self.posn-HEADER_W) .. ((self.posn-HEADER_W)+self.size))
    }
}



// debug printer for a Lump
impl fmt::Debug for Lump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}<0x{:X}-0x{:X}>", self.name, self.posn, self.posn+self.size)
    }
}


// end structs/lump.rs
