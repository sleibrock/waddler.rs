// main.rs
// the main entry point for the program

extern crate waddler;

use std::process::exit;
use std::env::args;

use waddler::svgmap::programs::svgmap_entrypoint;
use waddler::etc::programs::{info_entrypoint, debuglumps_entrypoint};


fn main() {
    let mut arg_iter = args();
    arg_iter.next();


    if arg_iter.len() == 0 {
        println!("waddler: No args supplied");
        exit(-1);
    }

    let subcom = match arg_iter.next() {
        Some(x) => x,
        None    => {
            println!("waddler: no subcommand given");
            exit(-2);
        },
    };
    println!("Subcommand is: {}", subcom);


    // list subcommands here
    // a subcommand function should return Result<u8, String>
    let result = match subcom.as_str() {
        "info"       => info_entrypoint(&mut arg_iter), 
        "lumps"      => debuglumps_entrypoint(&mut arg_iter),
        "svgmap"     => svgmap_entrypoint(&mut arg_iter), 
        "texdebug"   => Err(String::from("not implemented")),
        _            => Err(String::from("no matching program found")),
    };

    // unpack the Result<u8, String> type and process errors
    match result {
        Err(e) => {
            println!("waddler: {}", e);
            exit(-100);
        },
        _ => {},
    }

    exit(0);
}
