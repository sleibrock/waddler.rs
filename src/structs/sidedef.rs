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
    pub fn new(dat: &[u8]) -> SideDef {
        if dat.len() != SIDEDEF_W {
            panic!(format!("Sidedef not given {} bytes", SIDEDEF_W));
        }

        let mut zero1 : usize = 11;
        let mut zero2 : usize = 19;
        let mut zero3 : usize = 27;
        while dat[zero1] == 0 { zero1 -= 1; }
        while dat[zero2] == 0 { zero2 -= 1; }
        while dat[zero3] == 0 { zero3 -= 1; }

        SideDef {
            x_offset:   u8_to_i16(dat[0], dat[1]),
            y_offset:   u8_to_i16(dat[2], dat[3]),
            sector:     u8_to_u16(dat[28], dat[29]),
            upper_tex:  u8_to_string(&dat[4..zero1+1]),
            lower_tex:  u8_to_string(&dat[12..zero2+1]),
            middle_tex: u8_to_string(&dat[20..zero3+1]),
        }
    }
}


// end structs/sidedef.rs
