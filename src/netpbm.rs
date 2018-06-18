// netpbm.rs

/// NetPBM image library
/// Used for rendering .PPM P6 files (binary, rgb)

use std::path::Path;
use std::io::Write;
use std::fs::File;
 
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
 
pub struct PPM {
    width:  u32,
    height: u32,
    size:   usize,
    data:   Vec<u8>,
}
 
impl PPM {
    pub fn new(height: u32, width: u32) -> PPM
    {
        let buf_size = (3 * height * width) as usize;
        let buffer = vec![0; buf_size];

        return PPM {
            height: height,
            width:  width,
            size:   buf_size,
            data:   buffer,
        };
    }
 
    fn get_offset(&self, x: u32, y: u32) -> Option<usize>
    {
        let offset = ((y * self.width * 3) + (x * 3)) as usize;
        if offset < self.size {
            return Some(offset);
        } 
        return None;
    }
    
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB>
    {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.data[offset];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB {r: r, g: g, b: b})
            },
            None => None
        }
    }
 
    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool
    {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            },
            None => false
        }
    }
 
    pub fn write_file(&self, filename: &str) -> Result<u8, String>
    {
        //let path = Path::new(filename);
        let mut file = match File::create(filename) {
            Ok(f) => f,
            Err(e) => panic!("HELP 3"),
        };
        let header = format!("P6 {} {} 255\n", self.width, self.height);

        match file.write(header.as_bytes()) {
            Ok(_) => {},
            Err(e) => { panic!("HELP"); }
        }

        match file.write(&self.data) {
            Ok(_) => {},
            Err(e) => { panic!("HELP 2"); }
        }
        
        return Ok(0);
    }
}





// end netpbm.rs
