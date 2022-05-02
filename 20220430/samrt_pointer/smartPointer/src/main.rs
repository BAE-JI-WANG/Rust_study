use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);
}
