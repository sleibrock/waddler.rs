// structs/seg.rs

pub struct Seg {
    pub start:     u16,
    pub end:       u16,
    pub angle:     i16,
    pub line_id:   u16,
    pub direction: u16,
    pub offset:    u16,
}


impl Seg {
    pub fn new(dat: &[u8]) -> Seg {
        Seg {
            start: 0,
            end: 0,
            angle: 0,
            line_id: 0,
            direction: 0,
            offset: 0,
        }
    }
}


// end structs/seg.rs
