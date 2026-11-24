fn main() {
    let mut x: i32 = 5;
    //shadowing
    let x = x + 2;
    let x = x * 5;

    println!("Value of x : {}", x);
}
