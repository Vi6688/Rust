

mod hotel;

//absolute path as crate is the root folder path
use crate::hotel::hotel::Customer;

//relative path from the current module
use hotel::hotel::Ordering;

fn main() {

    let mut order = Ordering::new();

    let customer1 = Customer::new(String::from("Alice"), 4, 0);
    let customer2 = Customer::new(String::from("Bob"), 2, 5);
    let customer3 = Customer::new(String::from("Charlie"), 3, 0);

    order.addCustomer(customer1);
    order.addCustomer(customer2);
    order.addCustomer(customer3);

    println!("Total customers: {}", order.totalCustomers());
    println!(
        "Total customers waiting to be seated: {}",
        order.totalCustomersInWaiting()
    );
}
