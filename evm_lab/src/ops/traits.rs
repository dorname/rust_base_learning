pub trait Arithmetic {
    // 算术指令方法签名
    fn add(&mut self);
    fn mul(&mut self);
    fn sub(&mut self);
    fn div(&mut self);
    fn sdiv(&mut self);
    fn n_mod(&mut self);
    fn smod(&mut self);
    fn add_mod(&mut self);
    fn mul_mod(&mut self);
    fn exp(&mut self);
    fn sign_extend(&mut self);
}
pub trait Comparison {
    fn lt(&mut self);
    fn gt(&mut self);
    fn eq(&mut self);
    fn is_zero(&mut self);
    fn slt(&mut self);
    fn sgt(&mut self);
}

//位运算指令集
pub trait Bitwise {
    fn and(&mut self);
    fn or(&mut self);
    fn xor(&mut self);
    fn not(&mut self);
    fn byte(&mut self);
    fn shl(&mut self);
    fn shr(&mut self);
    fn sar(&mut self);
}

pub trait Memory {
    fn mstore(&mut self);
    fn mload(&mut self);
    fn msize(&mut self);
    fn mstore8(&mut self);
}


pub trait Storage {
    fn sstore(&mut self);
    fn sload(&mut self);
}
