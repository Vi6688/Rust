fn printString(someString: String) {
    println!("{someString}")
}
fn printNumber(someNumber: i32) {
    println!("{someNumber}")
}

fn main() {
    println!("Hello, world!");

    let s1: String = String::from("hello");
    let s2: String = s1.clone();
    println!("String value : {s1}");
    println!("String value : {s2}");

    let mut s = String::from("hello");

    s = s + &String::from("World");
    println!("String value : {s}");

    s = String::from("Hiii");
    let n = 10;

    //Here print string takes the ownership of the s so no longer can use the s in this scope
    //but you can use clone to use that in this scope

    // printString(s.clone());
    printString(s);
    printNumber(n);
    printNumber(n);

    //Pass by reference
    let str = String::from("lengthOfString");
    let strSize = calculateLength(&str);
    println!("strSize {strSize}");

    let mut strSize: i32 = 10;
    changeInt(&mut strSize);
    println!("Size {strSize}");

    //Slice Type
    let string = String::from("Hello, World!");
    let word = getTheFirstWord(&string);
    let firstWord = &string[0..word];
    println!("String : {firstWord}")
}

//Pass by reference
fn calculateLength(str: &String) -> usize {
    str.len()
}

fn changeInt(size: &mut i32) {
    *size = *size + 1;
}

fn getTheFirstWord(str : &String) -> (usize,usize)
{
    let mut firstWordStart = 0;
    let mut secondwordStart = 1;
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}