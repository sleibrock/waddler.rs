// etc/options.rs


use std::env::Args;



pub struct InfoOptions {
    pub help: bool,
    pub files: Vec<String>
}

pub struct DebugLumpsOptions {
    pub help:  bool,
    pub files: Vec<String>
}



impl DebugLumpsOptions {
    pub fn new(arg_iter: &mut Args) -> Result<DebugLumpsOptions, String> {
    let mut help = false;
    let mut files_buf : Vec<String> = Vec::new();

        let length : usize = arg_iter.len();
        if length == 0 {
            return Err(format!("no args supplied"));
        }

        let mut index : usize = 0;
        while index < length {
            let v = match arg_iter.next() {
                Some(arg) => arg,
                None => { return Err(format!("???")); },
            };

            match v.as_str() {
                "-h" => { help = true },
                _ => { files_buf.push(v.to_string()); },
            }
            index += 1;
        }

        return Ok(DebugLumpsOptions {
            help: help,
            files: files_buf,
        });
    }
}


impl InfoOptions {
    pub fn new(arg_iter: &mut Args) -> Result<InfoOptions, String> {
    let mut help = false;
    let mut files_buf : Vec<String> = Vec::new();

        let length : usize = arg_iter.len();
        if length == 0 {
            return Err(format!("debuglumps: no args supplied"));
        }


        let mut index : usize = 0;
        while index < length {
            let v = match arg_iter.next() {
                Some(arg) => arg,
                None      => { return Err(format!("???")); },
            };

            match v.as_str() {
                "-h" => { help = true },
                _    => { files_buf.push(v.to_string()); },
            }
            index += 1;
        }

        return Ok(InfoOptions {
            help: help,
            files: files_buf,
        });
    }
}



// end etc/options.rs
