fn main() {
    // immutable variable
    let x = 10;
    println!("x = {}", x);

    // mutable variable
    let mut y = 5;
    println!("y before = {}", y);
    y = 15;
    println!("y after = {}", y);

    // explicit types
    let a: i32 = 100;
    let b: f64 = 3.14;
    println!("a = {}, b = {}", a, b);

    // shadowing
    let z = 10;
    let z = z + 5;
    let z = z * 2;
    println!("z = {}", z);

    // constant
    const MAX_USERS: u32 = 1000;
    println!("MAX_USERS = {}", MAX_USERS);

    // scope
    let outside = 50;
    {
        let inside = 20;
        println!("inside = {}", inside);
        println!("outside = {}", outside);
    }

    // tuple destructuring
    let (x1, y1) = (3, 7);
    println!("x1 = {}, y1 = {}", x1, y1);

    // unused variable
    let _unused = 42;

    // next concept: data types
    let inferred_number = 42;
    let inferred_float = 2.5;
    println!("inferred_number = {}, inferred_float = {}", inferred_number, inferred_float);

    let person: (&str, u8, bool) = ("Hassan", 25, true);
    println!("name = {}, age = {}, active = {}", person.0, person.1, person.2);

    let (name, age, active) = person;
    println!("{} is {} years old: {}", name, age, active);

    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    println!("first = {}, last = {}", numbers[0], numbers[4]);

    let zeros = [0; 3];
    println!("zeros = {:?}", zeros);

    let mut scores = [90, 85, 88];
    scores[1] = 95;
    println!("scores = {:?}", scores);
}
