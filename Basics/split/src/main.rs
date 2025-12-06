pub(crate) fn main() {
    let string = "Hello, world!";

    let s1 = &string[0..5];
    let s2 = &string[5..10];
    let s3 = &string[0..string.len()];
    println!("{s1}{s2}");
    println!("{s3}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..3];
    for i in slice {
        println!("{i}")
    }
}
