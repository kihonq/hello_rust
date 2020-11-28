pub fn main() {
    println!("\n/============= Operator =============/\n");

    let mut a = 0;
    a += 1;
    println!("{} can be +=,-=,*=,%=,/= but not ++ or --", a);
    
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 1.3;
    let b_cubed = f64::powi(b, 3);
    let b_cube_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed is {}", b, b_cubed);
    println!("{} cubed is {}", b, b_cube_pi);
}