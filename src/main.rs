struct User {
    active: bool,
    name: String,
    email: String,
    user_id: u8,
}

fn main() {
    println!("Hello, world!");

    //assigning an integer
    let x: i32 = 1;
    //assigning a float
    let y: f64 = 1.1;

    println!("X is {}, Y is {}", x, y);

    //assigning value to mutable integer
    let mut z: i32 = 1;

    println!("Z is {}", z);

    z = 2;

    println!("Z is now {}", z);

    //defining 2 variables and calling a function
    let a = 5;
    let b = 10;

    let c = add_nums(a,b);
    println!("The sum of {} and {} is {}", a,b,c);

    //assigning tuple and array
    let _my_tuple: (i32, i32, f64, bool) = (1, 2, 3.14, true);
    let my_array: [i8; 4] = [1, 2, 3, 4];

    //iterating through array witha for loop
    println!("Elements in my_array:");
    for array_element in my_array {
        println!("{}", array_element);
    }

    println!("");

    //slicing array
    println!("Elements in sliced array:");
    let sliced_array = &my_array[1..4];
    for array_element in sliced_array {
        println!("{}", array_element);
    }

    //control flow - Fizz, Buzz, FizzBuzz
    println!("");
    println!("Control Flow: Fizz, Buzz, FizzBuzz");
    println!("");
    let number: u8 = 10;
    println!("Number is {}", number);
    
    if number % 3 == 0 && number % 5 == 0 {
        println!("FizzBuzz");
    } else if number % 3 == 0 {
        println!("Fizz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", number);
    }

    //creating instance of a struct "User"
    println!("");

    let name = "Prab".to_string();
    let email = "sainip1@wit.edu".to_string();
    let user_id = 1;



    let mut prab = build_user(name, email, user_id);

    println!("Is {} active? {}", prab.name, prab.active);
    println!("Email: {}", prab.email);
    println!("User ID: {}", prab.user_id);
    prab.active = false;
    println!("Is {} still active? {}", prab.name, prab.active);
    println!("");

}

fn add_nums (x: i32, y: i32) -> i32{
    x + y
}

fn build_user (name: String, email: String, user_id: u8) -> User {
    User {
        active: true,
        name,
        email,
        user_id,
    }
}