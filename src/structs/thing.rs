// structs/thing.rs

use utils::{u8_to_i16, u8_to_u16};
use structs::constants::{DOOM_THING_W, HEXEN_THING_W};


/*
doom struct
0 	2 	x position
2 	2 	y position
4 	2 	Angle facing
6 	2 	DoomEd thing type
8 	2 	Flags 

hexen struct
0	2	Thing id (tid)
2	2	X position
4	2	Y position
6	2	Starting height
8	2	Angle facing
10	2	DoomEd thing type
12	2	Flags
14	1	Hexen action special
15	1	Argument 1
16	1	Argument 2
17	1	Argument 3
18	1	Argument 4
19	1	Argument 5 
*/
pub struct Thing {
    pub x:       i16,
    pub y:       i16,
    pub tid:     u16,
    pub height:  u16,
    pub angle:   i16,
    pub ed_type: u16,
    pub flags:   u16,
    pub action:  u8,
    pub args:    [u8; 5],
}


impl Thing {
    pub fn new(is_hexen: bool, dat: &[u8]) -> Thing {
        match is_hexen {
            true => {
                if dat.len() != HEXEN_THING_W {
                    panic!(format!("Thing not given {} bytes", HEXEN_THING_W));
                }
                Thing {
                    tid:     u8_to_u16( dat[0],  dat[1]),
                    x:       u8_to_i16( dat[2],  dat[3]),
                    y:       u8_to_i16( dat[4],  dat[5]),
                    height:  u8_to_u16( dat[6],  dat[7]),
                    angle:   u8_to_i16( dat[8],  dat[9]),
                    ed_type: u8_to_u16(dat[10], dat[11]),
                    flags:   u8_to_u16(dat[12], dat[13]),
                    action:  dat[14],
                    args:    [dat[15], dat[16], dat[17], dat[18], dat[19]],
                }
            },

            _ => {
                if dat.len() != DOOM_THING_W {
                    panic!(format!("Thing not given {} bytes", DOOM_THING_W));
                }
                Thing {
                    tid:     0,
                    x:       u8_to_i16(dat[0], dat[1]),
                    y:       u8_to_i16(dat[2], dat[3]),
                    angle:   u8_to_i16(dat[4], dat[5]),
                    ed_type: u8_to_u16(dat[6], dat[7]),
                    flags:   u8_to_u16(dat[8], dat[9]),
                    action:  0,
                    height:  0,
                    args:    [0,0,0,0,0],
                }
            },
        }
    }
}


// end structs/thing.rs
