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
        println!("No args supplied");
        exit(-1);
    }

    let subcom = arg_iter.next().expect("???");
    println!("Subcommand is: {}", subcom);


    // list subcommands here
    let result = match subcom.as_str() {
        "maps" => map_program(&arg_iter), 
        _      => Err(String::from("no matching program found")),
    };

    match result {
        Ok(_) => {},
        Err(e) => panic!(e),
    }

    exit(0);
}
