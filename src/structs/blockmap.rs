// structs/blockmap.rs

// TODO: finish blockmaps

pub struct BlockMap {
    pub x:       i16,
    pub y:       i16,
    pub rows:    u16,
    pub columns: u16,
}


impl BlockMap {
    pub fn new(dat: &[u8]) -> BlockMap
    {
        BlockMap {
            x:       u8_to_i16(&dat[0..2]),
            y:       u8_to_i16(&dat[2..4]),
            rows:    u8_to_u16(&dat[6..8])
            columns: u8_to_u16(&dat[4..6]),
        }
    }

    pub fn get_offset(&self, n: u32) -> usize
    {
        0
    }
}


// end structs/blockmap.rs
