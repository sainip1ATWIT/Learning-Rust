//Defining a struct Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    //instantiating struct rect1
    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };

    println!("The area of width: {} and height: {} is {}", rect1.width, rect1.height, area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}