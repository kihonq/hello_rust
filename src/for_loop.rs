pub fn main() {
    println!("\n/============= For Loop =============/\n");

    for x in -1..11 { // -1 to 10
        if x == 0 { continue }
        if x == 5 { break }

        println!("x is {}", x)
    }

    for (i, y) in (-100..-95).enumerate() { // -1 to 10
        println!("y is {}, indexed at {}", y, i);
    }
}
