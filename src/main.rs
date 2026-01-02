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

    // conditionals
    let number = 7;
    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number is 5");
    } else {
        println!("number is greater than 5");
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
        println!("loop count = {}", count);
    }

    // while
    let mut n = 3;
    while n > 0 {
        println!("while n = {}", n);
        n -= 1;
    }

    // for over a range
    for i in 0..5 {
        println!("for i = {}", i);
    }

    // for over an array
    let numbers = [10, 20, 30];
    for num in numbers {
        println!("array number = {}", num);
    }

    // match (similar to switch)
    let day = 3;
    let day_name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid",
    };
    println!("day_name = {}", day_name);

    // enums
    // defining an enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    // using the enum
    let move1 = Direction::Up;
    let move2 = Direction::Left;

    // match with enum
    match move1 {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }

    match move2 {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }

    // enum with data
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };

    // pattern matching with data
    match msg1 {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to {},{},{}", r, g, b),
    }

    match msg2 {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::Chang
}
