// netpbm.rs

/// NetPBM image library
/// Used for rendering .PPM P6 files (binary, rgb)

pub struct PPM {
    pub width:  u64,
    pub height: u64,
    pub buf: Vec<u8>,
}


impl PPM {
    pub fn new(rows: u64, cols: u64) -> PPM {
        let mut buffer : Vec<u8> = Vec::new();


        return PPM {
            width: rows,
            height: cols,
            buf: buffer,
        };
    }

    pub fn set_pix(x: u64, y: u64) -> Result<u8, String>
    {
        Ok(0)
    }
}







// end netpbm.rs
