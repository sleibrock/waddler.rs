// structs/constants.rs

/// Widths for each type of Doom lump struct
/// These will be used to check the size of a data packet
/// before creating a struct. If the size of the data slice
/// does not match their respective widths, error out
pub const SEG_W           : usize = 12;
pub const LUMP_W          : usize = 16;
pub const HEADER_W        : usize = 12;
pub const VERTEX_W        : usize =  4;
pub const SECTOR_W        : usize = 26;
pub const SSECTOR_W       : usize =  4;
pub const SIDEDEF_W       : usize = 30;


/// Between Doom and Hexen, some structs changed in size for
/// additional features, so check for these widths when
/// working with structs of these families
pub const DOOM_LINEDEF_W  : usize = 14;
pub const HEXEN_LINEDEF_W : usize = 16;

pub const DOOM_THING_W    : usize = 10;
pub const HEXEN_THING_W   : usize = 20;


/// Textures
pub const MAPPATCH_W      : usize = 10;


/// A WAD can either be an id Software WAD, or a "player" WAD
/// the IWAD will have all the primary information, and a PWAD
/// will "overwrite" the data in the IWAD with new info (or add more)
/// If the WAD number does not match these two values, who knows what
/// the heck the file given was?
pub const IWAD_NUMBER     : u32   = 1145132873;
pub const PWAD_NUMBER     : u32   = 1145132880;



// end structs/constants.rs
