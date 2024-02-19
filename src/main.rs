mod enum_learning;
mod io_learning;
mod macro_learning;
mod closure_learning;
mod lifetime_learning;
mod new_type_learning;
mod generic_trait_learning;
mod ownership_learning;
mod thread_learning;
mod variable_mut_learning;
mod smart_points;
mod stack_learning;
mod array_learning;
// use p2p::add;
fn main() {
    println!("Hello, world!");
    // io_learning::read_file_test();
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
    // let nums:Vec<i32> = (0..100).collect();
    // println!("{:?}",nums);
    let arr:Vec<_> = ",32".split(',').collect();
    
    println!("{:?}",arr);
}
