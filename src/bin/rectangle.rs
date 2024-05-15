fn main() {
    let width = 30;
    let height = 50;

    println!("The area of a rectangle height: {} and width: {} is {}", height, width, area(height, width));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}