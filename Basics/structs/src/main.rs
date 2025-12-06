#[allow(non_snake_case)]

mod definitions;
use definitions::calculateRectangleArea;
use definitions::createCircle;
use definitions::Circle;
use definitions::Rectangle;

fn main() {
    let scale: f32 = 2.0;
    let rectangle1 = Rectangle {
        length: dbg!(scale * 10.00),
        height: 5.1,
    };
    let area = calculateRectangleArea(&rectangle1);

    println!("Details of an rectangle: {rectangle1:?}");
    println!("The area of the rectangle is: {0}", area);

    let circle1 = createCircle(1.0);
    println!("Details of the circle {circle1:?}")
}
