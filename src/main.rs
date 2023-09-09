struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (width1, height1);
    println!("The area of the rectangle is {} square pixels.", dimensions_to_area(rect1));
}

fn area(width: u32, height: u32) -> u32{
    width * height
}
fn dimensions_to_area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}