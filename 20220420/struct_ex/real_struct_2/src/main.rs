#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

fn main() {
    let rect1 = Rectangle {length: 50, width: 30};

    println!("rect is {:?}", rect1);
}
