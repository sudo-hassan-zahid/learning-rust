pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn div(a: i32, b: i32) -> i32 {
    if b == 0{
        panic!("Cannot divide by zero")
    }
    a / b
}