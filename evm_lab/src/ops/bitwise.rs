use crate::evm::Evm;
use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use num_bigint::BigUint;
use num_traits::{zero, one};

impl Bitwise for Evm {
    
    fn and(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop().unwrap();
        let unit_b = self.stack.pop().unwrap();
        let mut logger = LogTemplate::new_two_cal(
            "AND".to_owned(),
            "&".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = unit_a.0;
        let b = unit_b.0;
        let result = a & b;
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push((result,0u8));
    }

    fn or(&mut self) {
    }

    fn xor(&mut self) {
    }

    fn not(&mut self) {
    }

    fn shl(&mut self) {

    }
    fn shr(&mut self) {

    }
    fn sar(&mut self) {

    }
    fn byte(&mut self) {
        
    }
}

#[test]
fn test_and() {
    let bytes = vec![0x60, 0x08, 0x60, 0x06, 0x03, 0x60, 0x02, 0x16];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}