#[derive(Copy, Clone)]
pub enum Vegetables {
    Carrot,
    Broccoli,
    Spinach,
    Potato,
    Tomato,
}
impl Vegetables {
    pub fn size() -> usize {
        5
    }
}
impl From<i32> for Vegetables {
    fn from(num: i32) -> Self {
        match num {
            0 => Vegetables::Carrot,
            1 => Vegetables::Broccoli,
            2 => Vegetables::Spinach,
            3 => Vegetables::Potato,
            4 => Vegetables::Tomato,
            _ => panic!("Invalid number for Vegetable"),
        }
    }
}

pub enum Fruits {
    Apple,
    Banana,
    Orange,
    Mango,
    Grape,
}
