// svgmap/programs.rs


use std::env::Args;

use svg::*;
use structs::linedef::LineDef;
use structs::level::Level;
use structs::wad::Wad;

use utils::flip_u64;
use utils::{dir_name, path_str, make_dir};

use wadparse::parse;
use svgmap::options::SvgmapOptions;


/// Pick a line color for a given &LineDef struct
fn line_color(line: &LineDef, cdoors: bool, invert: bool) -> Color
{
    match cdoors {
        false => match line.one_sided {
            true => match invert {
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
                true => match invert {
                    true => Color::White,
                    _    => Color::Black,
                },
                _ => Color::Grey,
            }
        }
    }
}


/// Convert a &Level object into an SVG document struct
fn level_to_svg(lev: &Level, opts: &SvgmapOptions) -> SVG
{
    // iter through vertices to find min/max bounds
    let mut min_x : i16 = 0; let mut min_y : i16 = 0;
    let mut max_x : i16 = 0; let mut max_y : i16 = 0;
    for vert in &lev.vertices
    {
        if min_x == 0 && max_x == 0 && min_y == 0 && max_y == 0 {
            min_x = vert.x; max_x = vert.x;
            min_y = vert.y; max_y = vert.y;
        } else {
            if vert.x > max_x {
                max_x = vert.x;
            } else if vert.x < min_x {
                min_x = vert.x;
            }

            if vert.y > max_y {
                max_y = vert.y;
            } else if vert.y < min_y {
                min_y = vert.y;
            }
        }
    }

    let shift_x = 0 - min_x as i32;
    let shift_y = 0 - min_y as i32;

    let padding : u64 = 50;

    let mx = (max_x as i32) + shift_x;
    let my = (max_y as i32) + shift_y;

    let vx = mx + (2 * padding as i32);
    let vy = my + (2 * padding as i32);

    let base_canvas_size : f64 = opts.size as f64;
    let cx : u64;
    let cy : u64;
    if vx > vy {
        let r = base_canvas_size / vx as f64;
        cx = (vx as f64 * r) as u64;
        cy = (vy as f64 * r) as u64;
    } else {
        let r = base_canvas_size / vy as f64;
        cx = (vx as f64 * r) as u64;
        cy = (vy as f64 * r) as u64;
    }

    let mut buf = SVG::new(cx, cy, vx as u64, vy as u64);

    // check if we want a transparent background
    // else put in a filled background rect first
    if !opts.transparent {
        buf.add_object(Box::new(SVGRect::new(
            0,
            0,
            vx as u64,
            vy as u64,
            match opts.inverted {
                true => Color::Black,
                _    => Color::White,
            }
        )));
    }

    for linedef in &lev.linedefs
    {
        let a = &lev.vertices[linedef.start];
        let b = &lev.vertices[linedef.end];

        let ax = ((a.x as i32) + shift_x) as u64;
        let ay = ((a.y as i32) + shift_y) as u64;
        let bx = ((b.x as i32) + shift_x) as u64;
        let by = ((b.y as i32) + shift_y) as u64;

        buf.add_object(Box::new(SVGLine::new(
            padding + flip_u64(ax, 0),
            padding + flip_u64(ay, my as u64),
            padding + flip_u64(bx, 0),
            padding + flip_u64(by, my as u64),

            match linedef.one_sided {
                true => 7,
                _    => 5,
            },

            line_color(linedef, true, true),
        )));
    }

    return buf;
}


/// The main function to use to generate all SVG maps from a WAD
/// Requires a name (of the wad), a WAD struct, and options
pub fn make_maps_from_wad(
    fname: &str,
    wad:   &Wad,
    opts:  &SvgmapOptions
) -> Result<u8, String>
{
    let wad_dir_name = dir_name(fname, "data");
    let dir_made = make_dir(&wad_dir_name);
    
    if dir_made && opts.verbose {
        println!("exportmaps: wad directory made");
    }
    
    
    for lev in &wad.levels {
        println!("Current level: {}", &lev.name);
        let mut svg_thing = level_to_svg(&lev, &opts);
        let output_path = path_str(&wad_dir_name, &lev.name, "svg");
        
        match svg_thing.to_file(&output_path) {
            Err(e) => {
                return Err(format!("exportmaps: {}", e));
            },
            _ => {},
        }

        if opts.verbose {
            println!("svgmap: rendered map to {}", output_path);
        }
    }
    
    if opts.verbose {
        println!("svgmap: finished rendering maps for {}", fname);
    }
    
    return Ok(0);
}


pub fn svgmap_entrypoint(args: &mut Args) -> Result<u8, String>
{
    // create the options
    let opts = match SvgmapOptions::new(args) {
        Ok(o) => o,
        Err(e) => { return Err(format!("svgmap: {}", e)); },
    };

    opts.print();

    for fname in &opts.files {
        println!("File: {}", fname);
        let wad = match parse(fname) {
            Ok(w)  => w,
            Err(e) => { return Err(format!("svgmap: {}", e)); }, 
        };

        match make_maps_from_wad(fname, &wad, &opts) {
            Err(e) => { return Err(format!("svgmap: {}", e)); },
            Ok(_) => {
                if opts.verbose {
                    println!("Rendered wad {} successfully", fname);
                }
            },
        }
    }
    
    return Ok(0);
}

// end svgmap/programs.rs
