fn another_function1() {
    println!("Another function.");
}

fn another_function2(x: i32, y: String) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}
fn another_function3(x: i32) -> y {
    let y = {
        let z = x / 2;
        //should not have semicolon at the end of the expression to return the value
        z
    };
    println!("The value of y is: {y}");
    y
}
fn add(x: i32, y: i32) -> i32 {
    //Should not have semicolon at the end of the expression to return the value
    x + y
}

fn main() {
    another_function1();
    another_function2(10, String::from("Hello"));
    let sum = add(5, 10);
    println!("The sum is: {sum}");
    let result = another_function3(20);
    println!("The result is: {result}");
}
