// mapping/options.rs

/// Options to use for the mapping subprogram


use std::env::Args;


pub const HELP_STR: &'static str = "
Usage: waddler exportmaps [OPTION] ... [FILE] ...
Convert all levels from a list of WADs into SVG files
exported to matching directories of the original WAD filepath

  -h, --help         Show this help and exit
  -v, --version      Show program version and exit
  -V, --verbose      Toggle program verbosity
  -t, --transparent  Render images with no backgrounds
  -l, --lighting     Render images using sector lighting
  -i, --invert       Invert the colors (black bg, white fg)
  -s, --size [NUM]   Change the base canvas size
  -d, --doors        Color all keycard/skullkey doors

Examples:
  waddler exportmaps doom.wad        Exports all levels into './doom.wad.data/maps'
  waddler exportmaps -t heretic.wad  Exports all Heretic levels as transparent

More help can be found at <https://gitlab.com/sleibrock/waddler>
";



pub struct MappingOptions {
    pub help:         bool,
    pub size:         u64,
    pub files:        Vec<String>,
    pub version:      bool,
    pub verbose:      bool,
    pub lighting:     bool,
    pub inverted:     bool,
    pub transparent:  bool,
    pub color_doors:  bool,
}



impl MappingOptions {
    pub fn new(arg_iter: &mut Args) -> Result<MappingOptions, String> {

        let mut help = false;
        let mut verbose = false;
        let mut version = false;
        let mut size : u64 = 1024;
        let mut transparent = false;
        let mut lighting = false;
        let mut color_doors = false;
        let mut inverted = false;
        let mut files_buf : Vec<String> = Vec::new();




        let length : usize = arg_iter.len();
        if length == 0 {
            return Err(format!("exportmaps: No args supplied"));
        }

        let mut index : usize = 0;
        while index < length {
            let v = match arg_iter.next() {
                Some(arg) => arg,
                None      => { return Err(format!("???")); },
            };

            match v.as_str() {
                "-h"            => { help = true; }
                "--help"        => { help = true; }
                "-v"            => { version = true; }
                "--version"     => { version = true; }
                "-V"            => { verbose = true; }
                "--verbose"     => { verbose = true; }
                "-l"            => { lighting = true; }
                "--lighting"    => { lighting = true; }
                "-i"            => { inverted = true; }
                "--invert"      => { inverted = true; }
                "-d"            => { color_doors = true; }
                "--doors"       => { color_doors = true; }
                "-t"            => { transparent = true; }
                "--transparent" => { transparent = true; }


                "-s" => {
                    let v2 = match arg_iter.next() {
                        Some(arg) => arg,
                        None => {
                            return Err(format!("exportmaps: no size supplied"))
                        },
                    };

                    size = match v2.as_str().parse::<u64>() {
                        Ok(i) => i,
                        _ => {
                            return Err(format!("exportmaps: couldn't parse size"));
                        }
                    };
                    index += 1;
                },

                "--size" => {
                    let v2 = match arg_iter.next() {
                        Some(arg) => arg,
                        None => {
                            return Err(format!("exportmaps: no size supplied"))
                        },
                    };

                    size = match v2.as_str().parse::<u64>() {
                        Ok(i) => i,
                        _ => {
                            return Err(format!("exportmaps: couldn't parse size"));
                        }
                    };
                    index += 1;
                },

                _ => { files_buf.push(v.to_string()); },
            }

            index += 1;
        }

        Ok(MappingOptions {
            help:        help,
            size:        size,
            files:       files_buf,
            version:     version,
            verbose:     verbose,
            lighting:    lighting,
            inverted:    inverted,
            transparent: transparent,
            color_doors: color_doors,
        })
    } 
}

// end mapping/options.rs
