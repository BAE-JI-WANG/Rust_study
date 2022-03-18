fn main() {
    let _reference_to_nothing = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

