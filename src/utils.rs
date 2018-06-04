// utils.rs


use std::ops::Range;

pub fn packet(start: usize, width: usize) -> Range<usize> {
    (start..(start + width))
}


pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    (a as u16)+((b as u16)<<8)
}

pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32)+((b as u32)<<8)+((c as u32)<<16)+((d as u32)<<24)
}

pub fn u8_to_i16(a: u8, b: u8) -> i16 {
    u8_to_u16(a, b) as i16
}

pub fn u8_to_i32(a: u8, b: u8, c: u8, d: u8) -> i32 {
    u8_to_u32(a, b, c, d) as i32
}

pub fn u8_to_string(datslice: &[u8]) -> String {
    String::from_utf8_lossy(datslice).to_string()
}



#[cfg(test)]
mod tests {
    use utils::*;

    const DATA1 : [u8; 2] = [0, 0];
    const DATA2 : [u8; 2] = [255, 255];


    #[test]
    fn test_u16_to_i16() {
        let testvalue = u8_to_i16(DATA1[0], DATA1[1]);
        assert_eq!(testvalue, 0, "Conversion fail");
    }
}

// end utils.rs
