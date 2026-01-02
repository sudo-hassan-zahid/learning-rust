// Public function
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Public struct
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    // Public constructor
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: String::from(name),
            age,
        }
    }

    // Public method
    pub fn greet(&self) {
        println!("Hi, my name is {} and I am {} years old", self.name, self.age);
    }
}
