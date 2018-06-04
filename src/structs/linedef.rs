// structs/linedef.rs

use std::fmt;
use utils::{u8_to_i16, u8_to_u16};
use structs::constants::{DOOM_LINEDEF_W, HEXEN_LINEDEF_W};


pub struct LineDef {
    pub tag:       u16,
    pub end:       usize,
    pub left:      i16,
    pub start:     usize,
    pub right:     i16,
    pub flags:     u16,
    pub stype:     u16,
    pub args:      [u8; 6],
    pub is_hexen:  bool,
    pub one_sided: bool,
}


impl LineDef {
    pub fn new(is_hexen: bool, dat: &[u8]) -> LineDef {
        let right = u8_to_i16(dat[12], dat[13]);
        let left  = u8_to_i16(dat[14], dat[15]);
        let is_one_sided = left==-1 || right==-1;

        match is_hexen {
            true => {
                if dat.len() != HEXEN_LINEDEF_W {
                    panic!(format!("LineDef not given {} bytes", HEXEN_LINEDEF_W));
                }

                LineDef {
                    stype: 0,
                    tag:   0,
                    end:   u8_to_u16(dat[2],  dat[3]) as usize,
                    left:  left, 
                    start: u8_to_u16(dat[0],  dat[1]) as usize,
                    right: right, 
                    flags: u8_to_u16(dat[4],  dat[5]),
                    args:  [dat[6], dat[7], dat[8], dat[9], dat[10], dat[11]],
                    is_hexen: true,
                    one_sided: is_one_sided,
                }
            },

            _ => {
                if dat.len() != DOOM_LINEDEF_W {
                    panic!(format!("LineDef not given {} bytes", DOOM_LINEDEF_W));
                }

                LineDef {
                    tag:   u8_to_u16(dat[8],   dat[9]),
                    end:   u8_to_u16(dat[2],   dat[3]) as usize,
                    left:  left,
                    start: u8_to_u16(dat[0],   dat[1]) as usize,
                    right: right,
                    flags: u8_to_u16(dat[4],   dat[5]),
                    stype: u8_to_u16(dat[6],   dat[7]),
                    args:  [0, 0, 0, 0, 0, 0],
                    is_hexen: false,
                    one_sided: is_one_sided,
                }
            }
        }
    }
}


// debug printer for a Lump
impl fmt::Debug for LineDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LineDef<start: {}, end: {}>", self.start, self.end)
    }
}


// end structs/linedef.rs
