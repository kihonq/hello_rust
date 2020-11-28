pub fn main() {
    println!("\n/============= While & Loop =============/\n");

    let mut x = 1;
    while x < 500 {
        x *= 2;
        if x == 128 {
            continue;
        }

        println!("x is {}", x);
    }
    
    let mut y = 1;
    loop {
        y *= 2;
        println!("y is {}", y);

        if y == 1 << 10 { // 1^10
            break;
        }
    }
}
