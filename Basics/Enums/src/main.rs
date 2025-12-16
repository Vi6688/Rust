#[derive(PartialEq)]
enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Months {
    fn print_months(&self) {
        match self {
            Months::January => println!("January"),
            Months::February => println!("February"),
            Months::March => println!("March"),
            Months::April => println!("April"),
            Months::May => println!("May"),
            Months::June => println!("June"),
            Months::July => println!("July"),
            Months::August => println!("August"),
            Months::September => println!("September"),
            Months::October => println!("October"),
            Months::November => println!("November"),
            Months::December => println!("December"),
        }
    }
    fn asString(&self) -> String {
        match self {
            Months::January => String::from("January"),
            Months::February => String::from("February"),
            Months::March => String::from("March"),
            Months::April => String::from("April"),
            Months::May => String::from("May"),
            Months::June => String::from("June"),
            Months::July => String::from("July"),
            Months::August => String::from("August"),
            Months::September => String::from("September"),
            Months::October => String::from("October"),
            Months::November => String::from("November"),
            Months::December => String::from("December"),
        }
    }
}
impl From<i32> for Months {
    fn from(num: i32) -> Self {
        match num {
            0 => Months::January,
            1 => Months::February,
            2 => Months::March,
            3 => Months::April,
            4 => Months::May,
            5 => Months::June,
            6 => Months::July,
            7 => Months::August,
            8 => Months::September,
            9 => Months::October,
            10 => Months::November,
            11 => Months::December,
            _ => panic!("Invalid month number!"),
        }
    }
}

fn main() {
    let month = Months::January;
    let mut i = 0;
    loop {
        let name : Months= i.into();
        println!("Month in string: {}", Months::from(i).asString());
        i += 1;
        let monthEnum : Months = i.into();
        if monthEnum == Months::December {
            break;
        }
    }
}
