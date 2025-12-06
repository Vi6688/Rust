// #[derive(Debug)] allows us to print the structure using the :?
// :? is an example of a formatting trait
#[allow(non_snake_case)]


#[derive(Debug)]
pub struct Rectangle {
    pub length: f32,
    pub height: f32,
}
pub fn calculateRectangleArea(rectangle: &Rectangle) -> f32 {
    rectangle.length * rectangle.height
}

pub const PI: f32 = 3.14159;

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
    pub diameter: f32,
    pub area: f32,
}
pub fn createCircle(radius: f32) -> Circle {
    Circle {
        radius: radius,
        diameter: radius * 2.0,
        area: 2.0 * PI * radius,
    }
}

pub struct User {
    pub active: bool,
    pub name: String,
    pub email: String,
    pub id: u32,
}

#[derive(Debug)]
pub struct Axis {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct color(pub i32, pub i32, pub i32);

pub fn display(user1: User) {
    println!("{0}", user1.active);
    println!("{0}", user1.email);
    println!("{0}", user1.name);
    println!("{0}", user1.id);
}
pub fn exmpleUsage() {
    let user1 = User {
        active: true,
        name: String::from("Hii"),
        email: String::from("hii@gmail.com"),
        id: 12345,
    };
    display(user1);

    let black = color(0, 0, 0);
    let axis1 = Axis {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    println!("Details of the axis1 : {axis1:?}");
}
