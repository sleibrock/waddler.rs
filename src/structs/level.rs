// structs/level.rs

use std::fmt;
use utils::packet;
use structs::linedef::LineDef;
use structs::vertex::Vertex;
use structs::constants::{DOOM_LINEDEF_W, HEXEN_LINEDEF_W, VERTEX_W};


pub struct Level {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub linedefs: Vec<LineDef>,
}


impl Level {
    pub fn new(name: &String,
               vert_raw: &[u8],
               ld_raw: &[u8],
               is_hexen: bool
    ) -> Level {

        // start initializing vectors for the lumps
        let mut vertices : Vec<Vertex>  = Vec::new();
        let mut linedefs : Vec<LineDef> = Vec::new();


        // determine the widths of each struct needed
        let ld_w : usize = match is_hexen {
            true => HEXEN_LINEDEF_W,
            _    => DOOM_LINEDEF_W,
        };

        /*
        let thing_w : usize = match is_hexen {
            true => HEXEN_THING_W,
            _    => DOOM_THING_W,
        };
        */

        // start parsing raw data and initializing structs
        let mut off : usize = 0;
        while off < vert_raw.len() {
            vertices.push(Vertex::new(&vert_raw[packet(off, VERTEX_W)]));
            off += VERTEX_W;
        }

        off = 0;
        while off < ld_raw.len() {
            linedefs.push(LineDef::new(is_hexen, &ld_raw[packet(off, ld_w)]));
            off += ld_w;
        }


        /*
        offset = 0;
        while off < things_raw.len() {
            things.push(Thing::new(is_hexen, &things_raw[packet(off, thing_w)]));
            off += thing_w;
        } 
        */

        // return a new Level struct with each lump vector
        Level {
            name: name.to_owned(),
            vertices: vertices,
            linedefs: linedefs,
        }
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
