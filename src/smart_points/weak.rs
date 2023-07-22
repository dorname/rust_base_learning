use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc,Weak};
struct Car{
    name:String,
    wheels:RefCell<Vec<Weak<Wheel>>>
}
struct Wheel{
    id:i32,
    car:Rc<Car>
}
#[test]
fn test(){
    let car:Rc<Car> = Rc::new(
        Car {
            name: "Changcheng".to_string(),
            wheels:RefCell::new(vec![])
        }
    );
    let wheel_one:Rc<Wheel>= Rc::new(Wheel{id:1,car:Rc::clone(&car)});
    let wheel_two:Rc<Wheel> = Rc::new(Wheel { id: (2), car: (Rc::clone(&car)) }); 
    //可变借用赋值放在内部代码块中可避免与后续的不可变借用产生冲突
    { 
    let mut wheels = car.wheels.borrow_mut();
    wheels.push(Rc::downgrade(&wheel_one));
    wheels.push(Rc::downgrade(&wheel_two));
    }
    // println!("{}",car.name);
    //不可变借用car.wheels.borrow()
    for wheel_weak in car.wheels.borrow().iter() {
    // for wheel_weak in wheels.iter() {
        let wl = wheel_weak.upgrade().unwrap();
        println!("车轮id{},车名name{}",wl.id,wl.car.name);
    }

    println!("{}",Rc::downgrade(&wheel_one).weak_count());
    
}