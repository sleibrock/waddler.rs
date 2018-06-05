// utils.rs


use std::ops::Range;
use std::fs::create_dir;


/// Data slicing function to yield a Range<usize> type
/// which will be used to slice arrays of u8's
pub fn packet(start: usize, width: usize) -> Range<usize> {
    (start..(start + width))
}


/// Little Endian conversion functions
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



/// File and directory utilities
fn dir_name(dname: &str, extn: &str) -> String {
    format!("{}.{}", dname, extn)
}


fn make_dir(dname: &str) -> bool {
    match create_dir(format!("{}", dname)) {
        Ok(_) => true,
        _     => false,
    }
}


/// Flip a u64 value across an axis
/// If the axis is set to zero, return the value
fn flip_u64(v: u64, m: u64) -> u64 {
    match m == 0 {
        true => v,
        _    => m - v,
    }
}


/// tests
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
