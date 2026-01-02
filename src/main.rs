use std::collections::HashMap;

// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Trait definition
trait Summary {
    fn summarize(&self) -> String;
}

// Implementing trait for a struct
struct Article {
    headline: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        // Using content to remove dead_code warning
        format!(
            "{} by {}. Preview: {}",
            self.headline,
            self.author,
            &self.content[..std::cmp::min(10, self.content.len())]
        )
    }
}

// Enums
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Struct
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }
}

// Functions returning Result
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err(String::from("Cannot divide by zero")) } else { Ok(a / b) }
}

fn try_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("division by zero"));
    }
    Ok(a / b)
}

mod utils;
mod math;

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

    // using enums
    let move1 = Direction::Up;
    let move2 = Direction::Left;

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

    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };

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
        Message::ChangeColor(r, g, b) => println!("Change color to {},{},{}", r, g, b),
    }

    // struct usage
    let person1 = Person {
        name: String::from("Hassan"),
        age: 25,
    };

    let mut person2 = Person {
        name: String::from("Ali"),
        age: 30,
    };

    println!("{} is {} years old", person1.name, person1.age);
    person2.age += 1;
    println!("{} is now {} years old", person2.name, person2.age);

    person1.greet();
    person2.greet();
    person2.have_birthday();
    println!("After birthday, {} is {} years old", person2.name, person2.age);

    // Option type
    let some_number: Option<i32> = Some(10);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("We have a number: {}", n),
        None => println!("No number found"),
    }

    match no_number {
        Some(n) => println!("We have a number: {}", n),
        None => println!("No number found"),
    }

    println!("some_number = {}", some_number.unwrap());
    println!("no_number or default = {}", no_number.unwrap_or(0));

    // Result type
    let result1 = divide(10, 2);
    let result2 = divide(10, 0);

    match result1 {
        Ok(val) => println!("10 / 2 = {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match result2 {
        Ok(val) => println!("10 / 0 = {}", val),
        Err(e) => println!("Error: {}", e),
    }

    let r = try_divide(20, 4).unwrap();
    println!("20 / 4 = {}", r);

    // Using generic function
    let numbers = vec![34, 50, 25, 100, 65];
    let max_number = largest(&numbers);
    println!("The largest number is {}", max_number);

    let chars = vec!['y', 'm', 'a', 'q'];
    let max_char = largest(&chars);
    println!("The largest char is {}", max_char);

    // Using generic struct
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.2, y: 3.4 };
    println!(
        "int_point: ({}, {}), float_point: ({}, {})",
        int_point.x,
        int_point.y,
        float_point.x,
        float_point.y
    );

    // Using trait
    let article = Article {
        headline: String::from("Rust is awesome"),
        author: String::from("Hassan"),
        content: String::from("Learn Rust step by step"),
    };

    println!("Article summary: {}", article.summarize());

    // Vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("Vector: {:?}", v);

    println!("First element: {}", v[0]);
    match v.get(1) {
        Some(val) => println!("Second element: {}", val),
        None => println!("No second element"),
    }

    for val in &v {
        println!("Value: {}", val);
    }

    // Strings
    let mut s = String::from("Hello");
    s.push_str(" world");
    println!("String: {}", s);

    // HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score = {}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores
        .entry("Alice")
        .and_modify(|e| {
            *e += 10;
        })
        .or_insert(0);
    println!("Alice's updated score = {}", scores["Alice"]);

    scores.remove("Bob");
    println!("Scores after removal: {:?}", scores);

    // Construct unused Direction variants
    let _ = Direction::Down;
    let _ = Direction::Right;

    // Construct unused Message variants
    let _ = Message::Quit;
    let _ = Message::ChangeColor(0, 0, 0);

    // Closures (anonymous functions)
    let add = |a: i32, b: i32| -> i32 { a + b };
    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    // Closure capturing environment
    let x = 10;
    let multiply = |y| y * x; // captures x
    println!("5 * x = {}", multiply(5));

    // Simple iterator over a vector
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("number = {}", num);
    }

    // Using map to transform elements
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|n| n * 2)
        .collect();
    println!("Doubled = {:?}", doubled);

    // Using filter to select elements
    let numbers = vec![10, 15, 20, 25, 30];
    let even: Vec<i32> = numbers
        .iter()
        .filter(|n| *n % 2 == 0)
        .cloned()
        .collect();
    println!("Even numbers = {:?}", even);

    // Using enumerate to get index + value
    let letters = vec!['a', 'b', 'c'];
    for (i, letter) in letters.iter().enumerate() {
        println!("Index {} = {}", i, letter);
    }

    // Using fold to reduce to single value
    let sum: i32 = numbers.iter().fold(0, |acc, n| acc + n);
    println!("Sum = {}", sum);

    // Combining iterator adapters
    let combined: Vec<i32> = (1..10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    println!("Squares of even numbers 1..9 = {:?}", combined);

    // Sorting iterator results
    let mut nums = vec![3, 5, 1, 4, 2];
    nums.sort();
    println!("Sorted nums = {:?}", nums);

    // Using closures with custom sort
    let mut people = vec!["Alice", "Bob", "Charlie"];
    people.sort_by(|a, b| b.len().cmp(&a.len())); // sort by length descending
    println!("People sorted by name length = {:?}", people);

    // Iterator for Option
    let some_val = Some(5);
    let doubled_val: Option<i32> = some_val.map(|v| v * 2);
    println!("Some doubled value = {:?}", doubled_val);

    // Iterator for Result
    let res: Result<i32, &str> = Ok(10);
    let res_squared = res.map(|v| v * v);
    println!("Result squared = {:?}", res_squared);

    let person = utils::Person::new("Hassan", 30);
    person.greet();

    math::add(1, 2);
    math::multiply(3, 4);
    math::sub(10, 5);
    math::div(100, 10);
    math::div(10, 0);

    utils::greet("Hassan")
}
