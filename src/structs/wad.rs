// structs/wad.rs


use std::fmt;
use std::ops::{Range, RangeFrom};

use utils::{u8_to_u32, u8_slice};
use structs::constants::{HEADER_W, LUMP_W};
use structs::constants::{IWAD_NUMBER, PWAD_NUMBER};
use structs::lump::Lump;
use structs::level::Level;
//use structs::seg::Seg;
//use structs::sector::Sector;
//use structs::subsector::Subsector;
//use structs::sidedef::SideDef;
//use structs::blockmap::BlockMap;
//use structs::reject::Reject;
//use structs::behavior::Behavior;
//use structs::texture::TextureLump;


pub struct WadHeader {
    pub is_wad:     bool,
    pub wadtype:    u32,
    pub numlumps:   usize,
    pub lumpaddr:   usize,
}


pub struct Wad {
    pub name:     String,
    pub header:   WadHeader,
    pub lumps:    Vec<Lump>,
    pub levels:   Vec<Level>,
    pub is_hexen: bool,
}


impl WadHeader {
    pub fn new(dat: &[u8]) -> WadHeader
    {
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

    // Methods used to calculate the ranges of the two WAD data pools
    pub fn data_range(&self) -> Range<usize> { ((HEADER_W .. self.lumpaddr)) }
    pub fn lump_range(&self) -> RangeFrom<usize> { (self.lumpaddr ..) }
}


/// Print formatter for a Header
impl fmt::Debug for WadHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
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


/// Wad initializer method. Takes in two slices and a header
/// and returns a full Wad structure
impl Wad {
    pub fn new(
        n:        &str,
        hd:       WadHeader,
        lumpd:    &[u8],
        dat:      &[u8],
    ) -> Result<Wad, String> {
        if lumpd.len() == 0 || dat.len() == 0 {
            return Err(format!("No data given to Wad::new()"));
        }

        let mut d_count : usize = 0;
        let mut lumps   : Vec<Lump>  = Vec::new();
        let mut levels  : Vec<Level> = Vec::new();

        // create a vector of Lumps from the lump data
        let mut l_offset = 0;
        let mut is_hexen = false;
        while l_offset < lumpd.len()
        {
            // generate a lump from the slice
            let pkt = u8_slice(l_offset, LUMP_W, &lumpd);
            let l = Lump::new(&pkt);

            // check if the lump is a BEHAVIOR lump from Hexen
            if l.name.starts_with("BEHAVIOR") {
                is_hexen = true;
            }
            
            // append to the lump vector
            lumps.push(l);
            l_offset += LUMP_W;
        }

        // if there are BEHAVIOR lumps, then we need an additional
        // data count target to account for it
        let data_count_target = match is_hexen {
            true => 4,
            _    => 4,
        };


        let mut index : usize = 0;
        let mut clevel    : usize = 0;
        let mut cverts    : usize = 0;
        let mut clines    : usize = 0;
        let mut cthings   : usize = 0;
        //let mut csectors  : &Lump = &lumps[0];
        //let mut csubsectors : &Lump = &lumps[0];
        //let mut csidedefs : &Lump = &lumps[0];


        while index < lumps.len()
        {
            let lump : &Lump = &lumps[index]; // get current lump
            
            if lump.is_level {
                clevel = index;
                d_count += 1;
            } else {
                match lump.name.as_str() {
                    "VERTEXES" => { cverts =  index;  d_count += 1; }
                    "LINEDEFS" => { clines =  index;  d_count += 1; }
                    "THINGS"   => { cthings = index;  d_count += 1; }
                    "SECTORS"  => {}
                    "SSECTORS" => {}
                    "SIDEDEFS" => {}
                    _ => {}
                }
            }

            if d_count == data_count_target {
                levels.push(Level::new(
                    &lumps[clevel].name,
                    u8_slice(lumps[cverts].start,  lumps[cverts].size, dat),
                    u8_slice(lumps[clines].start,  lumps[clines].size, dat),
                    u8_slice(lumps[cthings].start, lumps[cthings].size, dat),
                    is_hexen,
                ));
                d_count = 0;
            }
            index += 1;
        }

        return Ok(Wad {
            name: String::from(n),
            header: hd,
            lumps: lumps,
            levels: levels,
            is_hexen: is_hexen,
        });
    }


    pub fn print_levels(&self) {
        for x in &self.levels {
            println!("{:?}", x);
        }
    }

}

// end structs/wad.rs
