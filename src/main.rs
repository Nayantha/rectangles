#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        rectangle.width * rectangle.height
    }
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = Rectangle{ width: width1,  height: height1, };
    println!("{:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
