// use std::cell::Cell;

use std::cell::Cell;
use std::ops::Deref;





struct Grid {
    width:i32,
    height:Cell<i32>
}

impl Grid {
    fn new(&self,w:i32,h:i32) -> Self{
        Self{
            width:w,
            height:Cell::new(h)
        }
    }

}

impl Deref for Grid {
    type Target = (i32,i32);
    fn deref(&self) -> &Self::Target {
        // &(self.width, self.height)  cannot return reference to temporary value returns a reference to data owned by the current function
        Box::leak(Box::new((self.width, self.height.get())))
    }
}
#[test]
fn test(){
    let a = Cell::new(2);
    a.set(6);
    println!("get:{}",a.get());
    println!("take out:{}",a.take());
    println!("take in:{}",a.get());
    println!("replace out:{}",a.replace(5));
    println!("replace in:{}",a.get());
    //into_inner()会把a move掉
    println!("into_inner:{}",a.into_inner());
    // println!("replace in:{}",a.get());

    let grid =  Grid{
        width:100,
        height:Cell::new(200)
    };
    grid.height.set(600);
    println!("{:?}",*grid);

    // println!(grid)
}