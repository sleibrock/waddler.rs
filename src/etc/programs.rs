// etc/programs.rs


use std::env::Args;
use wadparse::parse;

use etc::options::{InfoOptions, DebugLumpsOptions};


pub fn info_entrypoint(args: &mut Args) -> Result<u8, String> {
    Err(format!("Not implemented"))
}

pub fn debuglumps_entrypoint(args: &mut Args) -> Result<u8, String> {

    let opts = match DebugLumpsOptions::new(args) {
        Ok(o) => o,
        Err(e) => { return Err(format!("debuglumps: {}", e)); },
    };

    for fname in &opts.files {
        let wad = match parse(fname) {
            Ok(w) => w,
            Err(e) => { return Err(format!("???")); },
        };

        println!("got a wad");
    }
    
    
    return Err(format!("debuglumps: not implemented"));
}

// end etc/programs.rs
