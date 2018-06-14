// structs/texture.rs

/// These structs represent the TEXTURE1 and TEXTURE2 lumps
/// as well as functions for handling parsing of texture blobs

use utils::{u8_to_u32, u8_to_u16, u8_to_string};
use structs::constants::MAPPATCH_W;

pub struct TextureLump {
    pub numtextures:  usize,
    pub offsets:      Vec<usize>,
    pub mtextures:    Vec<MapTexture>,
}

/*
0x00 	8 	name 	An ASCII string defining the name of the map texture. Only the characters A-Z (uppercase), 0-9, and [ ] - _ should be used in lump names. When a string is less than 8 bytes long, it should be null-padded to the eighth byte.
0x08 	4 	masked 	A boolean (0=false, 1=true) defining ?
0x0C 	2 	width 	A short integer defining the total width of the map texture.
0x0E 	2 	height 	A short integer defining the total height of the map texture.
0x10 	4 	columndirectory 	Obsolete, ignored by all DOOM versions
0x14 	2 	patchcount 	the number of map patches that make up this map texture
0x16 	10 * patchcount 	patches[ ] 	array with the map patch structures for this texture. (see next table) 
*/
pub struct MapTexture {
    pub name:            String,
    pub masked:          u32,
    pub width:           u16,
    pub height:          u16,
    pub columndirectory: u32,
    pub patchcount:      u16,
    pub patches:         Vec<MapPatch>,
}


/*
0x00 	2 	originx 	A short int defining the horizontal offset of the patch relative to the upper-left of the texture
0x02 	2 	originy 	A short int defining the vertical offset of the patch relative to the upper-left of the texture
0x04 	2 	patch 	A short int defining the patch number (as listed in PNAMES) to draw
0x06 	2 	stepdir 	A short int defining ?
0x08 	2 	colormap 	A short int defining ? 
*/
pub struct MapPatch {
    pub x_origin: u16,
    pub y_origin: u16,
    pub patch:    u16,
    pub stepdir:  u16,
    pub colormap: u16,
}


impl MapPatch {
    pub fn new(dat: &[u8]) -> MapPatch
    {
        if dat.len() != MAPPATCH_W {
            panic!("MapPatch not valid size");
        }

        return MapPatch {
            x_origin: 0,
            y_origin: 0,
            patch:    0,
            stepdir:  0,
            colormap: 0,
        }
    }
}


/// methods for the Texture Lump TEXTURE1 and TEXTURE2 
impl TextureLump {
    fn new(dat: &[u8]) -> TextureLump {
        let mut off : Vec<usize> = Vec::new();
        let mut tex : Vec<MapTexture> = Vec::new();

        let numtex = u8_to_u32(dat[0], dat[1], dat[2], dat[3]) as usize;
        let offstart = numtex*4 as usize;

        return TextureLump {
            numtextures: numtex as usize,
            offsets:     off,
            mtextures:   tex,
        };
    }
}

// end structs/texture.rs
