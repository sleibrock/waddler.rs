// structs/wad.rs


use std::fmt;
use std::ops::{Range, RangeFrom};

use utils::*;
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
    pub wadtype:    u32,
    pub numlumps:   usize,
    pub lumpaddr:   usize,
    pub data_range: Range<usize>, 
    pub lump_range: RangeFrom<usize>,
    
}


pub struct Wad {
    pub name:     String,
    pub header:   WadHeader,
    pub lumps:    Vec<Lump>,
    pub levels:   Vec<Level>,
    pub is_hexen: bool,
}


impl WadHeader {
    pub fn new(dat: &[u8]) -> Result<WadHeader, String> {
	// check if we were able to even receive data
        if dat.len() != HEADER_W {
	    return Err(format!("Header not given {} bytes", HEADER_W).into())
        }

	// check if we have a valid IWAD/PWAD value
        let wad = u8_to_u32(&dat[0..3]);
	if wad != IWAD_NUMBER || wad != PWAD_NUMBER {
	    return Err(format!("Not a valid WAD (wadid: {})", wad).into());
	}

	// get the number of lumps, and the lump address range
	let num_lumps = u8_to_usize(&dat[4..7]);
	let lump_addr = u8_to_usize(&dat[8..11]);

        Ok(WadHeader {
            wadtype:  wad, 
            numlumps: num_lumps,
            lumpaddr: lump_addr, 
	    data_range: (HEADER_W .. lump_addr),
	    lump_range: (lump_addr ..),
        })
    }

    // Methods used to calculate the ranges of the two WAD data pools
    pub fn data_range(&self) -> Range<usize> { HEADER_W .. self.lumpaddr }
    pub fn lump_range(&self) -> RangeFrom<usize> { self.lumpaddr .. }
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
	// check if the lump_data slice matches the header count
	// move this to the Wad builder
	if lumpd.len() != hd.numlumps * LUMP_W {
            return Err(String::from("Lump count does not match header"));
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
            let l = Lump::new(&pkt)?;

            // check if the lump is a BEHAVIOR lump from Hexen
	    // This BEHAVIOR logic is only used by the Hexen game engine 
            if l.name.starts_with("BEHAVIOR") {
                is_hexen = true;
            }
            
            // append to the lump vector
            lumps.push(l);
            l_offset += LUMP_W;
        }

        // if there are BEHAVIOR lumps, then we need an additional
        // data count target to account for it
	// TODO: why do I have this here? am I stupid?
        let data_count_target = match is_hexen {
            true => 4,
            _    => 4,
        };


        let mut index : usize = 0;
        let mut clevel : usize = 0;
        let mut cverts : usize = 0;
        let mut clines : usize = 0;
        let mut cthings : usize = 0;
        //let mut csectors  : &Lump = &lumps[0];
        //let mut csubsectors : &Lump = &lumps[0];
        //let mut csidedefs : &Lump = &lumps[0];

        while index < lumps.len()
        {
            let lump = &lumps[index]; // get current lump
            
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
                )?);
                d_count = 0;
            }
            index += 1;
        }

        Ok(Wad {
            name: n.into(),
            header: hd,
            lumps: lumps,
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
