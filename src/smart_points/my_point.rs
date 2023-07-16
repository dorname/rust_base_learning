use std::ops::Deref;
//定义一个泛型参数的元组结构体
struct MYBOX<T>(T);
//为元组结构体实现一个创建实例的方法
impl<T> MYBOX<T>  {
    fn new(x:T) -> Self{
        Self(x)
    }
}
impl<T> Deref for MYBOX<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl <T> Drop for MYBOX<T> {
    fn drop(&mut self) {
        println!("ALREADY DROP")
    }
}
#[test]
fn test(){
    let x = MYBOX::new(21);
    println!("{:?}",*x);
}