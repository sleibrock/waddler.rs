// structs/linedef.rs

use std::fmt;
use utils::*;
use structs::constants::*;


/// A LineDef is a line connected by two Vertices in Doomspace
/// The :start and :end properties are indices in the Level's
/// VERTEXES lump, :left and :right are signed ints to determine
/// whether there is a something to the left or to the right of it
/// (if any of these are equal to -1, then it is one-sided (wall))
///
/// A LineDef's structure will change based on whether the game is
/// Hexen or not, so special methods should be added for safety,
/// or proper data packing should be in place to avoid method defs
pub struct LineDef {
    pub tag:       u16,
    pub end:       usize,
    pub left:      i16,
    pub start:     usize,
    pub right:     i16,
    pub flags:     u16,
    pub spectype:  u16,
    pub args:      [u8; 6],
    pub is_hexen:  bool,
    pub one_sided: bool,
}


impl LineDef {
    pub fn new(is_hexen: bool, dat: &[u8]) -> Result<LineDef, String> {
        // match based on whether the game is Hexen or not
        match is_hexen {
            true => {
                if dat.len() != HEXEN_LINEDEF_W {
                    return Err(format!("LineDef not proper bytes").into());
                }
                let right = u8_to_i16(&dat[12..13]);
                let left  = u8_to_i16(&dat[14..15]);
                let is_one_sided = left==-1 || right==-1;

                Ok(LineDef {
                    spectype: dat[6] as u16,
                    tag:      0,
                    end:      u8_to_usize(&dat[2..3]),
                    left:     left, 
                    start:    u8_to_usize(&dat[0..1]),
                    right:    right, 
                    flags:    u8_to_u16(&dat[4..5]),
                    args:     [dat[6], dat[7], dat[8], dat[9], dat[10], dat[11]],
                    is_hexen: true,
                    one_sided: is_one_sided,
                })
            },

            _ => {
                if dat.len() != DOOM_LINEDEF_W {
                    return Err(format!("LineDef not given proper bytes").into());
                }
                let left  = u8_to_i16(&dat[10..11]);
                let right = u8_to_i16(&dat[12..13]);
                let is_one_sided = left==-1 || right==-1;

                Ok(LineDef {
                    tag:       u8_to_u16(&dat[8..9]),
                    end:       u8_to_usize(&dat[2..3]),
                    left:      left,
                    start:     u8_to_usize(&dat[0..1]),
                    right:     right,
                    flags:     u8_to_u16(&dat[4..5]),
                    spectype:  u8_to_u16(&dat[6..7]),
                    args:      [0, 0, 0, 0, 0, 0],
                    is_hexen:  false,
                    one_sided: is_one_sided,
                })
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
