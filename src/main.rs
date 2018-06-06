// main.rs
// the main entry point for the program

extern crate waddler;

use std::process::exit;
use std::env::args;

use waddler::mapping::mapper::map_program;


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
        "info"       => Err(String::from("Incomplete")),
        "debuglumps" => Err(String::from("Incomplete")), 
        "exportmaps" => map_program(&arg_iter), 
        _            => Err(String::from("no matching program found")),
    };

    // unpack the Result<u8, String> type and process errors
    match result {
        Ok(_) => {},
        Err(e) => {
            println!("waddler: {}", e);
            exit(-100);
        },
    }

    exit(0);
}
