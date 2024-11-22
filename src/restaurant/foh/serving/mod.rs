use crate::restaurant;

pub fn take_order() {
    println!("Take order");
}

pub fn serve_order() {
    println!("Serve order");
}

pub fn take_payment() {
    println!("Thanks for your visit at {}", restaurant::display_name());
}

pub fn take_complaint() {
    println!("Take complaint");
}
