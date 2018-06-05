// mapping/mapper.rs

use std::fs::create_dir;
use std::env::Args;

use structs::linedef::LineDef;
use structs::level::Level;
use structs::wad::Wad;

// end mapping/mapper.rs



/*
pub fn make_maps_from_wad(fname: &str,
                          wad: &Wad,
                          opts: &Options
) -> Result<u8, String> {
    let wad_dir_name = "";
    let dir_made = false;



    return Err(String::from("bleh"));
}
*/



pub fn map_program(args: &Args) -> Result<u8, String> {
   Ok(0)
}

// end mapping/mapper.rs
