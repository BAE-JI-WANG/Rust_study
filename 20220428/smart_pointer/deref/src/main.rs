fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x = {}", x);
    println!("&x = {}", &x);
    println!("y = {}", y);
    println!("*y = {}", *y);
    println!("&y = {}", &y);
}
