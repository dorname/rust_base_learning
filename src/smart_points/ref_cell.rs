use std::cell::RefCell;

#[test]
fn test() {
    let rc = RefCell::new(4);

    // 使用 borrow 方法获取不可变引用，注意先获取不可变引用，由于借用规则，不可再通过rc获取可变引用
    {
        let rc_data = rc.borrow();
        println!("first try: {}", *rc_data);
    }
    // 在内部作用域中获取可变引用
    {
    let mut mutable_data = rc.borrow_mut();
    *mutable_data += 10;
    println!("The updated value is: {}", *mutable_data);
    }
    
    let rc_data_one = rc.borrow();
    println!("final try: {}", *rc_data_one);
}
