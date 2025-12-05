use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let integer = 10;
    println!("The integer is: {integer}");

    let float = 3.14;
    println!("The float is: {float}");

    let double = 2.71828 * 1E6;
    println!("The double is: {double}");

    let string = "Hello, world!";
    println!("The string is: {string}");

    let mut boolean = true;
    println!("The boolean is: {boolean}");

    boolean = false;
    println!("The boolean is: {boolean}");

    const MAX_POINTS: u32 = 100_000;
    println!("The constant MAX_POINTS is: {MAX_POINTS}");

    const PI: f64 = 3.141592653589793;
    println!("The constant PI is: {PI}");

    let mut spaces = "Hello";
    println!("The spaces string is: '{spaces}'");

    let number_of_spaces = spaces.len();
    println!("The number of spaces is: {number_of_spaces}");

    //if we specify the type it wont type cast even if the value is a floating point
    let unsigned_integer: u32 = "255".parse().expect("Not a number!");
    println!("The unsigned integer is: {unsigned_integer}");

    let signed_integer: i16 = "-255".parse().expect("Not a number!");
    println!("The signed integer is: {signed_integer}");

    let mut a: i8 = 100;
    let b: i8 = 20;
    a = a + b; //this will give error if the value exceeds the range
    println!("The sum of a and b is: {a}");

    let hexaDecimal: u32 = 0b11111111;
    println!("The hexadecimal value is: {hexaDecimal}");

    let boolean: bool = true;
    println!("The boolean value is: {boolean}");

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let x = tuple.1;
    println!("The value of tup is : {x}");

    let mut array: [i32; 5] = [3; 5];
    // array = [1, 2, 3, 4, 5];

    let mut index: String = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let mut index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number");
            0
        }
    };
    index = if index >= array.len() { 0 } else { index };
    let x = array[index];
    println!("The {index} element of the array is: {x}");
}
