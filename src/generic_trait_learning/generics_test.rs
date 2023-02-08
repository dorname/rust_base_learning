use std::ops::Add;

fn add_i8(a:i8,b:i8) -> i8 {
    a+b
}
fn add_i16(a:i16,b:i16) -> i16 {
    a+b
}
fn add_i32(a:i32,b:i32) -> i32 {
    a+b
}
fn add_test(){
    println!("{}",add_i8(1,2));
    println!("{}",add_i16(1,2));
    println!("{}",add_i32(1,2));
    println!("{}",add(1,2));
}

fn add<T:std::ops::Add + Add<Output = T>>(a:T,b:T) -> T {
    a+b
}
#[test]
fn test() {
    add_test();

}