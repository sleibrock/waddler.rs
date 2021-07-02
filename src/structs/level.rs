// structs/level.rs

/// The Level struct contains vectors for all Level data
/// ie: the VERTEXES lump for a level gets converted to Vec<Vertex>
/// ie2: the LINEDEFS lump gets converted to Vec<LineDef>
/// Parsing of structs is done in Level::new()
/// Adding more data to the level requires adding more params to the
/// Level::new() function

use std::fmt;
use utils::u8_slice;
use structs::linedef::LineDef;
use structs::vertex::Vertex;
use structs::thing::Thing;
use structs::constants::{VERTEX_W};
use structs::constants::{DOOM_LINEDEF_W, HEXEN_LINEDEF_W};
use structs::constants::{DOOM_THING_W, HEXEN_THING_W};


/// Level struct, which contains a name (String)
/// and several vectors for Level Data lumps
/// Shared lumps will be stored at the WAD level (structs/wad.rs)
pub struct Level {
    pub name:     String,
    pub vertices: Vec<Vertex>,
    pub linedefs: Vec<LineDef>,
    pub things:   Vec<Thing>,
}


impl Level {
    pub fn new(
        name:      &String,
        vert_raw:  &[u8],
        ld_raw:    &[u8],
        thing_raw: &[u8],
        is_hexen: bool
    ) -> Result<Level, String> {
        // start initializing vectors for the lumps
        let mut vertices : Vec<Vertex>  = Vec::new();
        let mut linedefs : Vec<LineDef> = Vec::new();
        let mut things   : Vec<Thing>   = Vec::new();


        // determine the widths of each struct needed
        let ld_w = match is_hexen {
            true => HEXEN_LINEDEF_W,
            _    => DOOM_LINEDEF_W,
        };

        let thing_w = match is_hexen {
            true => HEXEN_THING_W,
            _    => DOOM_THING_W,
        };

        // start parsing raw data and initializing structs
        let mut off : usize = 0;
        while off < vert_raw.len() {
            vertices.push(Vertex::new(u8_slice(off, VERTEX_W, &vert_raw))?);
            off += VERTEX_W;
        }

        off = 0;
        while off < ld_raw.len() {
            linedefs.push(LineDef::new(is_hexen, u8_slice(off, ld_w, &ld_raw))?);
            off += ld_w;
        }

        off = 0;
        while off < thing_raw.len() { 
            things.push(Thing::new(is_hexen, u8_slice(off, thing_w, &thing_raw))?);
            off += thing_w;
        } 

        // return a new Level struct with each lump vector
        Ok(Level {
            name: name.to_owned(),
            vertices: vertices,
            linedefs: linedefs,
            things:   things,
        })
    }
}


impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Level<{}, verts: {}, lines: {}>",
               self.name, self.vertices.len(), self.linedefs.len()
        )
    }
}

// end structs/level.rs
