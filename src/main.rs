fn main() {
    // Immutable variables (default in Rust)
    let x = 10;
    println!("x = {}", x);

    // Mutable variables
    let mut y = 5;
    println!("y before = {}", y);

    y = 15; // allowed because y is mutable
    println!("y after = {}", y);

    // Explicit type annotations
    let a: i32 = 100;      // 32-bit signed integer
    let b: f64 = 3.14;     // 64-bit floating point

    println!("a = {}, b = {}", a, b);

    // Shadowing (re-declaring a variable)
    let z = 10;
    let z = z + 5;         // shadows the previous z
    let z = z * 2;         // shadows again

    println!("z = {}", z); // 30

    // Constants
    const MAX_USERS: u32 = 1000;
    println!("MAX_USERS = {}", MAX_USERS);

    // Scope example
    let outside = 50;

    {
        let inside = 20;
        println!("inside = {}", inside);
        println!("outside (from inner scope) = {}", outside);
    }


    // Destructuring (tuple variables)
    let (x1, y1) = (3, 7);
    println!("x1 = {}, y1 = {}", x1, y1);

    // Unused variables (prefix with underscore)
    let _unused = 42;
}
