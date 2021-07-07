// utils.rs


use std::ops::Range;
use std::fs::create_dir;


/// Data slicing function to yield a Range<usize> type
/// which will be used to slice arrays of u8's
pub fn packet(start: usize, width: usize) -> Range<usize> {
    start..(start + width)
}

pub fn u8_slice(start: usize, width: usize, dat: &[u8]) -> &[u8] {
    &dat[start..(start + width)]
}


/*
/// Little Endian conversion functions
pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    (a as u16)+((b as u16)<<8)
}

pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (a as u32)+((b as u32)<<8)+((c as u32)<<16)+((d as u32)<<24)
}

pub fn u8_to_u32size(a: u8, b: u8, c: u8, d: u8) -> usize {
    ((a as u32)+((b as u32)<<8)+((c as u32)<<16)+((d as u32)<<24)) as usize
}

pub fn u8_to_i16(a: u8, b: u8) -> i16 {
    u8_to_u16(a, b) as i16
}

pub fn u8_to_i32(a: &[u8]) -> i32 {
    u8_to_u32(slice) as i32
}
*/

pub fn u8_to_string(datslice: &[u8]) -> String {
    String::from_utf8_lossy(datslice).to_string()
}

pub fn u8_to_u32(slice: &[u8]) -> u32 {
    slice.into_iter().enumerate().map(
	|(i, x)| {
	    println!("x{}: {}", i, x);
	    (*x as u32) << (i*8)
	}
    ).sum()
}

pub fn u8_to_usize(slice: &[u8]) -> usize {
    u8_to_u32(slice) as usize
}

pub fn u8_to_u16(slice: &[u8]) -> u16 {
    u8_to_u32(slice) as u16
}

pub fn u8_to_i32(slice: &[u8]) -> i32 {
    u8_to_u32(slice) as i32
}

pub fn u8_to_i16(slice: &[u8]) -> i16 {
    u8_to_u32(slice) as i16
}


/// Finding zeroes
pub fn find_zero_from_right(datslice: &[u8], init: usize) -> usize {
    for x in (0..init).into_iter().rev() {
	if datslice[x] == 0 { return x; }
    }
    return 0;
}

pub fn find_zero_from_left(datslice: &[u8], init: usize) -> usize {
    for x in (0..init).into_iter() {
	if datslice[x] == 0 { return x; }
    }
    return 0;
}


/// File and directory utilities
pub fn dir_name(dname: &str, extn: &str) -> String {
    format!("{}.{}", dname, extn)
}


pub fn make_dir(dname: &str) -> bool {
    match create_dir(format!("{}", dname)) {
        Ok(_) => true,
        _     => false,
    }
}


pub fn path_str(dir: &str, lname: &str, extn: &str) -> String {
    format!("{}/{}.{}", dir, lname, extn)
}


/// Flip a u64 value across an axis
/// If the axis is set to zero, return the value
pub fn flip_u64(v: u64, m: u64) -> u64 {
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
    const DATA3 : [u8; 4] = [255, 255, 255, 255];


    #[test]
    fn test_u16_to_i16() {
        let testvalue = u8_to_i16(&DATA1[0..2]);
        assert_eq!(testvalue, 0, "u8->i16 messed up");
	let tv2 = u8_to_u16(&DATA2[0..2]);
	assert_eq!(tv2, 65535, "u8->u16 messed up");
	let tv3 = u8_to_u16(&DATA1[0..2]);
	assert_eq!(tv3, 0, "u8->u16 messed up");
	let tv4 = u8_to_i16(&DATA2[0..2]);
	assert_eq!(tv4, -1, "u8->i16 messed up");
    }
}

// end utils.rs
