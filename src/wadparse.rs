// wadparse.rs

/// This file exposes a function that will attempt to parse a WAD file
/// given a file path. It will attempt to convert the file into a
/// usable WAD struct complete with levels and other structs

use std::fs::File;
use std::io::Read;

use structs::wad::{Wad, WadHeader};
use structs::constants::{HEADER_W, LUMP_W};


/// Parse a Wad file into a Wad struct
/// :fname: must be a valid path to a file
pub fn parse(fname: &str) -> Result<Wad, String>
{
    // begin opening the file
    let mut f = match File::open(fname) {
        Ok(nf) => nf,
        _ => { return Err(String::from("Could not open file")); },
    };
    let mut all_bytes : Vec<u8> = Vec::new();
    match f.read_to_end(&mut all_bytes) {
        Ok(_) => {},
        _ => return Err("Failed to read all bytes from the file".into()),
    };

    // initialize the header with the first 12 bytes
    let header = WadHeader::new(&all_bytes[0..HEADER_W])?;

    // slice the data and lump bytes into different pools
    let core_data = &all_bytes[header.data_range()];
    let lump_data = &all_bytes[header.lump_range()];

    // TODO: wrapping this in an Ok() makes rust fail to build
    return Wad::new(fname, header, &lump_data[..], &core_data[..]);
}

// end
