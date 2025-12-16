
pub struct Customer {
    pub name : String,
    pub noOfGuests : u32,
    pub tableNumber : u32,
}
impl Customer {
    pub fn new(name: String, noOfGuests: u32, tableNumber: u32) -> Customer {
        Customer {
            name,
            noOfGuests,
            tableNumber,
        }
    }
    pub fn hasSeated(&self) -> bool {
    return self.tableNumber != 0;
    }
}

pub struct Ordering {
    pub Customers : Vec<Customer>,
    
}
impl Ordering {
    pub fn new() -> Ordering {
        Ordering {
            Customers : Vec::new(),
        }
    }
    pub fn addCustomer(&mut self, customer: Customer) {
        self.Customers.push(customer);
    }
    pub fn totalCustomers(&self) -> usize {
        self.Customers.len()
    }
    pub fn totalCustomersInWaiting(&self) -> usize {
        let mut count = 0;
        for customer in &self.Customers {
            if !customer.hasSeated() {
                count += 1;
            }
        }
        count
    }
}