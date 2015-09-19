extern crate byteorder;

use std::fs::{File};
use std::io::{Read};
use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let mut fh = File::open("blk00000.dat").unwrap();
     
    loop {
        
        let magic_id = fh.read_u32::<LittleEndian>().unwrap();
        let header_length = fh.read_u32::<LittleEndian>().unwrap();
        
        println!("magic_id: {:x}", magic_id);
        println!("header_length: {}", header_length);
        
        let mut buf = vec![0; header_length as usize];
        let _ = fh.read(&mut buf);

    }
}
