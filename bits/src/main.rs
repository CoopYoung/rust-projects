#![allow(warnings)]   
use std::io;

/*TODO: Increase the size to 128 bits! */

//Global array for tracking types of bytes
static byte_array:[&str; 7] = ["b", "KB", "MB", "GB", "TB", "PB", "EB"];


fn main() {

    let mut byte_lims:[u64; 7] = [0,0,0,0,0,0,0];
    init_bytes(&mut byte_lims);
    
    loop 
    {
        println!("\n\n\nConvert a 64 bit integer into it's corresponding [KB,MB,GB,TB,PB,EB]!\n");

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .unwrap();
        let bit: u64 = input.trim().parse().unwrap();

        let mut lim:f64 = 0.0;
        let mut suffix:&str = "";
        (lim, suffix) = bits_to_bytes(bit, &mut byte_lims);
        println!("{bit} bits = {lim} {suffix}");
    }
    
    
}

/*
 //TODO:implement the struct

struct Bytes{
    size: u32,
    byte_limits: [u64; 7],
    idx: u32,
}
*/


fn init_bytes(byte_lims: &mut [u64; 7])  
{
    let mut x:u64 = 1;
    let mut byte_limit_idx = 0;
    let mut res:u64;
    while x < 64
    {
        if x % 10 == 0
        {

            res = 2u64.pow((x as u64).try_into().unwrap())  as u64;
            byte_lims[byte_limit_idx] = res; 
            byte_limit_idx += 1;

        }
        else if x == 63
        {
            res = 2u64.pow((x as u64).try_into().unwrap())  as u64;
            byte_lims[byte_limit_idx] = res; 
        }
        x += 1;
    }

}
fn bits_to_bytes(bits: u64, byte_lims: &mut [u64;7]) -> (f64,&str) {
    let mut idx = 0;
    let mut suffix: &str = ""; 
    let mut res = 0.0; 
    //bits=1280 -> 1.2 KB
    //bits=9712850 -> 9.71285 MB

    for lim in byte_lims
    {
        if bits < *lim
        {
            suffix = byte_array[idx]; 
            idx *= 10;
            let converted:u64 = 2u64.pow((idx as u64).try_into().unwrap()) as u64; 
            res = bits as f64 / converted as f64; 
            return (res, suffix);
        }
        idx += 1;
    }
    return (res, suffix);

    

}
