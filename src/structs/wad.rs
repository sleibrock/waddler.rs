// structs/wad.rs


use std::fmt;
use std::ops::{Range, RangeFrom};

use utils::{u8_to_u32,u8_slice};
use structs::constants::{HEADER_W, IWAD_NUMBER, PWAD_NUMBER};
use structs::lump::Lump;
use structs::level::Level;
//use structs::seg::Seg;
//use structs::sector::Sector;
//use structs::subsector::Subsector;
//use structs::sidedef::SideDef;
//use structs::blockmap::BlockMap;
//use structs::reject::Reject;
//use structs::behavior::Behavior;



pub struct WadHeader {
    pub is_wad:     bool,
    pub wadtype:    u32,
    pub numlumps:   usize,
    pub lumpaddr:   usize,
}



pub struct Wad {
    pub name:     String,
    pub header:   WadHeader,
    pub levels:   Vec<Level>,
    pub is_hexen: bool,
}


impl WadHeader {
    pub fn new(dat: &[u8]) -> WadHeader {
        if dat.len() != HEADER_W {
            panic!(format!("Header not given {} bytes", HEADER_W));
        }

        let wad : u32 = u8_to_u32(dat[0], dat[1], dat[2], dat[3]);

        WadHeader {
            wadtype:  wad, 
            numlumps: u8_to_u32(dat[4], dat[5],  dat[6],  dat[7]) as usize,
            lumpaddr: u8_to_u32(dat[8], dat[9], dat[10], dat[11]) as usize,
            is_wad: wad == IWAD_NUMBER || wad == PWAD_NUMBER,
        }
    }

    pub fn data_range(&self) -> Range<usize> { ((HEADER_W .. self.lumpaddr)) }
    pub fn lump_range(&self) -> RangeFrom<usize> { (self.lumpaddr ..) }
}



impl fmt::Debug for WadHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WadHeader<{}, lumps: {}, addr: 0x{:X}>",
               match self.wadtype {
                   IWAD_NUMBER => "IWAD",
                   PWAD_NUMBER => "PWAD",
                   _           => "UNKNOWN",
               },
               self.numlumps,
               self.lumpaddr,
        )
    }
}



impl Wad {
    pub fn new(
        n:        &str,
        hd:       WadHeader,
        lumps:    &Vec<Lump>,
        dat:      &[u8],
        is_hexen: bool
    ) -> Result<Wad, String> {
        if lumps.len() == 0 {
            return Err(format!("No lumps given to Wad::new()"));
        }

        let mut clevel    : &Lump = &lumps[0];
        let mut cverts    : &Lump = &lumps[0];
        let mut clines    : &Lump = &lumps[0];
        let mut cthings   : &Lump = &lumps[0];
        //let mut csectors  : &Lump = &lumps[0];
        //let mut csubsectors : &Lump = &lumps[0];
        //let mut csidedefs : &Lump = &lumps[0];

        let mut data_count : usize  = 0;
        let mut levels : Vec<Level> = Vec::new();

        // if there are BEHAVIOR lumps, then we need an additiona
        let data_count_target = match is_hexen {
            true => 4,
            _    => 4,
        };


        for lump in lumps {
            if lump.is_level {
                clevel = lump;
                data_count += 1;
            } else {
                match lump.name.as_str() {
                    "VERTEXES" => { cverts = lump; data_count += 1; }
                    "LINEDEFS" => { clines = lump; data_count += 1; }
                    "THINGS"   => { cthings = lump; data_count += 1;}
                    "SECTORS"  => {}
                    "SSECTORS" => {}
                    "SIDEDEFS" => {}
                    _ => {}
                }
            }


            if data_count == data_count_target {
                levels.push(Level::new(
                    &clevel.name,
                    u8_slice(cverts.start, cverts.size, dat),
                    u8_slice(clines.start, clines.size, dat),
                    u8_slice(cthings.start, cthings.size, dat),
                    is_hexen,
                ));
                data_count = 0;
            }
        }

        Ok(Wad {
            name: String::from(n),
            header: hd,
            levels: levels,
            is_hexen: is_hexen,
        })
    }


    pub fn print_levels(&self) {
        for x in &self.levels {
            println!("{:?}", x);
        }
    }

}

// end structs/wad.rs
