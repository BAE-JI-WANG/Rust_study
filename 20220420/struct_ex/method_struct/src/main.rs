#[derive(Debug)]
struct Rectangle {
    lenth: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.lenth * self.width
    }
}

fn main() {
    let rect1 = Rectangle{lenth: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
