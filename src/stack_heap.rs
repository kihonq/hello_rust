#![allow(dead_code)]
use std::mem;

struct Coordinate {
    lat: f64,
    long: f64
}

fn origin() -> Coordinate {
    Coordinate {
        lat: 0.0,
        long: 0.0
    }
}

pub fn main() {
    println!("\n/============= Stack & Heap =============/\n");

    let c0 = origin();
    let c1 = Box::new(origin());

    println!("c0 takes up {} bytes", mem::size_of_val(&c0));
    println!("c1 takes up {} bytes", mem::size_of_val(&c1));
    
    let c2 = *c1;
    println!("c2 lat is {}, c2 long is {}", c2.lat, c2.long);
}