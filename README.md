# Struct in Rust
Rectangle is used to Demonstrate
<br>
* **Define Struct** <br>
  #[derive(Debug)] can be used to easily print struct on the output / CMD.
```rust
struct Rectangle {
width: u32, // fields
height: u32,
}
```

* **Implement Methods**
  <br>

```rust
impl Rectangle {
  fn area(&self) -> u32 { // self is calling itself
  self.width * self.height
  }
  fn can_hold(&self, rect: &Rectangle) -> bool {
  self.area() >= rect.area() && (
  (self.width >= rect.width && self.height >= rect.height) ||
  (self.width >= rect.height && self.height >= rect.width)
  )
  }
  fn make_square(size: u32) -> Rectangle{ // associated method - not calling self method
  Rectangle{width: size, height: size}
  }
  }
```
