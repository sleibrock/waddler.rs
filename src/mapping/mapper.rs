// mapping/mapper.rs

use std::fs::create_dir;
use std::env::Args;

use svg::*;
use structs::linedef::LineDef;
use structs::level::Level;
use structs::wad::Wad;

// end mapping/mapper.rs



fn line_color(line: &LineDef, color_doors: bool, inverted: bool) -> Color {
    match color_doors {
        false => match line.one_sided {
            true => match inverted {
                true => Color::White,
                _    => Color::Black,
            },
            _ => Color::Grey,
        },
        _ => match line.spectype {
            28 => Color::Red,    // red keycard
            33 => Color::Red,    // Red keycard stay open
            26 => Color::Blue,   // blue keycard
            32 => Color::Blue,   // blue keycard stay open
            27 => Color::Yellow, // yellow keycard stay open
            34 => Color::Yellow, // yellow keycard stay open
            _ => match line.one_sided {
                true => match inverted {
                    true => Color::White,
                    _    => Color::Black,
                },
                _ => Color::Grey,
            }
        }
    }
}


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
