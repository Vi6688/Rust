fn ifStatement() -> bool {
    let number = 100;

    //if statement syntax
    //Condition check is similar to python without semicolon
    //No parentheses needed around the condition
    //Execution block is defined using curly braces
    //Only boolean expressions are allowed in condition checks

    if number < 50 {
        println!("The number is less than 50");
    } else if number < 100 {
        println!("The number is less than 100");
    } else if number == 100 {
        println!("The number is equal to 100");
    } else {
        println!("The number is greater than 100");
        return false;
    }
    return true;
}

fn main() {
    println!("Hello, world!");
    let status = ifStatement();
    println!("The if statement returned: {status}");

    //Ternary operator
    let condition = true;
    let number = if condition { 10 } else { 10 };
    println!("The value of number is: {number}");

    let mut i = 0;
    loop {
        i += 1;
        println!("The value of i is: {i}");
        if i > 3 {
            break;
        }
    }

    //Loop with return value
    let mut counter = 0;

    let result = loop {
        counter += 1;

        //Break statement with value

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //Loop labels
    let mut count = 0;
    'outer_loop: loop {
        println!("Count: {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    //while loop
    let mut index = 0;
    while index < 5 {
        println!("Value of the index :{index}");
        index = index + 1;
    }

    let a: [u8; 6] = [0, 1, 2, 3, 4, 5];
    for item in a {
        println! {"Item in the array a :{item}"}
    }
}
