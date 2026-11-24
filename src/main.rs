fn main() {
    /*
    let mut x: i32 = 5;
    //shadowing
    let x = x + 2;
    let x = x * 5;
     */
    // Expression dans rust
    let x: i32 = {
        let y: i32 = 3;
        y
    };

    println!("Value of x : {}", x);
}
