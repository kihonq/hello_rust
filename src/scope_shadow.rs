pub fn main() {
    println!("\n/============= Scope & Shadow =============/\n");

    let a = 13;
    
    {
        println!("inside before declaring new a {}", a);
        let a = 10;
        println!("inside after declaring new a {}", a);
    }
    
    println!("outside is {}", a);
}