use std::mem;

fn main() {
    let unsigned_int: u8 = 123;
    println!("unsigned_int = {}", unsigned_int);
    
    let signed_int: i16 = -22222;
    println!("signed_int = {}", signed_int);    
    
    let mut mutable_int: i8 = 0;
    println!("mutable_int = {} before", mutable_int);
    mutable_int = -1;
    println!("mutable_int = {} after", mutable_int);

    let z: isize = 1000;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, runs at {}-bit OS", z, size_of_z, size_of_z*8);

    let character: char = '1';
    println!("{} is of char type, takes up {} bytes", character, mem::size_of_val(&character));

    let float: f32 = 1.001;
    println!("{} is of float type, takes up {} bytes", float, mem::size_of_val(&float));

    let default_float = 1.0;
    println!("{} is of float type, takes up {} bytes", default_float, mem::size_of_val(&default_float));

    let boolean: bool = false;
    println!("{} is of boolean type, takes up {} bytes", boolean, mem::size_of_val(&boolean));
}
