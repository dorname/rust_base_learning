use std::fmt;
fn print_and_return<T: fmt::Debug>(value: T) -> T {
    println!("Received value: {:?}", value);
    value
}
#[test]
fn test() {
    let number = 42;
    let result = print_and_return(number);
    println!("Returned value: {:?}", result);

    let message = "Hello, Rust!";
    let result = print_and_return(message);
    println!("Returned value: {:?}", result);
}
