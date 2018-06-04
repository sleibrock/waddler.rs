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
    pub fn new(name: &String, vert_raw: &[u8], ld_raw: &[u8], is_hexen: bool) -> Level {

        let mut vertices : Vec<Vertex> = Vec::new();
        let mut linedefs : Vec<LineDef> = Vec::new();


        let ld_width : usize = match is_hexen {
            true => HEXEN_LINEDEF_W,
            _    => DOOM_LINEDEF_W,
        };

        let mut offset : usize = 0;
        while offset < vert_raw.len() {
            vertices.push(Vertex::new(&vert_raw[packet(offset, VERTEX_W)]));
            offset += VERTEX_W;
        }

        offset = 0;
        while offset < ld_raw.len() {
            linedefs.push(LineDef::new(
                is_hexen,
                &ld_raw[packet(offset, ld_width)],
            ));
        }

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
