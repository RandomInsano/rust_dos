use alloc::format;
use rust_dos::dos::{
    console,
    datetime
};

#[allow(dead_code)]
pub(crate) fn print_test() {
    let date = datetime::Date::now();
    let output = format!("Test print date with print(): {:?}\n$", date);

    console::print(&output).unwrap();
    console::print("Hello from print_test()!\n$$$$").unwrap();
    assert!(console::print("Hello from print_test()!\n").is_err());
}