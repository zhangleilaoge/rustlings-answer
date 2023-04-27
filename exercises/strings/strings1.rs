// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

struct Person {
    name: String,
    age: u32,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
    let p = Person { name: String::from("Bob"), age: 20 };
    println!("{}", p);
}

fn current_favorite_color() -> String {
    // "blue".to_string()
    String::from("blue")
}

