// wadparse.rs

/// This file exposes a function that will attempt to parse a WAD file
/// given a file path. It will attempt to convert the file into a
/// usable WAD struct complete with levels and other structs

use std::fs::File;
use std::io::Read;

use utils::u8_slice;
use structs::wad::{Wad, WadHeader};
use structs::lump::Lump;
use structs::constants::{HEADER_W, LUMP_W};


pub fn parse(fname: &str) -> Result<Wad, String> {

    // begin opening the file
    let mut f = match File::open(fname) {
        Ok(nf) => nf,
        _      => { return Err(String::from("Could not open file")); },
    };
    let mut all_bytes : Vec<u8> = Vec::new();
    match f.read_to_end(&mut all_bytes) {
        Ok(_) => {},
        _     => panic!("Failed to read all bytes from the file"),
    };


    // initialize the header with the first 12 bytes
    let header = WadHeader::new(&all_bytes[0..HEADER_W]);
    if !header.is_wad {
        return Err(String::from(format!("{} is not a valid wad", &fname)));
    }
    println!("{:?}", header);

    // slice the data and lump bytes into different pools
    let core_data = &all_bytes[header.data_range()];
    let lump_data = &all_bytes[header.lump_range()];

    // vars for generating lumps
    let mut is_hexen : bool = false;
    let mut offset : usize = 0;
    let mut lumps : Vec<Lump> = Vec::new();

    // lump reading loop which increments the offset by 16
    while offset < lump_data.len() {
        let pkt = u8_slice(offset, LUMP_W, &lump_data);
//        let pkt = &lump_data[packet(offset, LUMP_W)];

        // generate a lump from the slice
        let l = Lump::new(&pkt);
        if l.name.starts_with("BEHAVIOR") {
            is_hexen = true;
        }

        // append to the lump vector
        lumps.push(l);
        offset += LUMP_W;
    }

    if lumps.len() != header.numlumps {
        return Err(String::from("Lump count does not match header"));
    }

    return Wad::new(fname, header, &lumps, &core_data[..], is_hexen);
}

// end
