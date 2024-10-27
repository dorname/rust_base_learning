use crate::evm::Evm;
use crate::utils::*;
use crate::ops::traits::Other;
use log::{info, logger};
use num_bigint::BigUint;
use num_traits::{ToPrimitive};
impl Other for Evm {
    fn sha3(&mut self){
        if self.stack.len()< 2 {
            panic!("Stack underflow");
        }
        fn get_data(memory: &Vec<u8>, offset: BigUint, size: BigUint) -> Vec<u8> {
            let start = offset.to_usize().unwrap();
            let end = (offset + size).to_usize().unwrap();
            memory[start..end].to_vec()
        }
        let offset = get_uint256(self.stack.pop().unwrap());
        let size = get_uint256(self.stack.pop().unwrap());
        let data = get_data(&self.memory, offset, size);
        let hash = keccak256(&data).to_vec();
        info!("sha3:{:?}",vec_to_hex_string(hash.clone()));
        self.stack.push((BigUint::from_bytes_be(&hash),0u8));        
    }
}

#[test]
fn test_sha3() {
    let excute_codes = "5F5F20";
    let bytes = hex::decode(excute_codes).unwrap();
    // let bytes = vec![0x61, 0xff,0x00];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}