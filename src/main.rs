#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() >= rect.area() && (
            (self.width >= rect.width && self.height >= rect.height) ||
                (self.width >= rect.height && self.height >= rect.width)
        )
    }
    fn make_square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = Rectangle{ width: width1,  height: height1, };
    println!("{:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect1? {}", rect1.can_hold(&rect1));
}
