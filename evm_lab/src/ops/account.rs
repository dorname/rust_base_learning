use crate::evm::Evm;
use crate::ops::traits::AccountTraits;
use crate::utils::*;
use log::{info, logger};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::io::Read;
/// TODO 后面实现区块链用户部分完善
impl AccountTraits for Evm {
    fn balance(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let addr_int = get_uint256(self.stack.pop().unwrap());
        let addr_str = format!("0x{}", vec_to_hex_string(addr_int.to_bytes_be()));
        let balance = get_account_db().get_account(addr_str).balance.clone();
        self.stack.push((balance, 0u8));
    }
    fn extcodecopy(&mut self) {
        if self.stack.len() < 4 {
            panic!("Stack underflow");
        }
        let addr_int = get_uint256(self.stack.pop().unwrap());
        let mem_offset = get_uint256(self.stack.pop().unwrap());
        let code_offset = get_uint256(self.stack.pop().unwrap());
        let lenght = get_uint256(self.stack.pop().unwrap());
        let addr_str = format!("0x{}", vec_to_hex_string(addr_int.to_bytes_be()));
        let offset = (lenght.clone() + mem_offset.clone()).to_usize().unwrap();
        let code_offset_len = (lenght.clone() + code_offset.clone()).to_usize().unwrap();
        let code = get_account_db().get_account(addr_str).code.clone()
            [code_offset.to_usize().unwrap()..code_offset_len]
            .to_vec();

        // if code.len() < 32 {
        //     // 如果字节长度不足 32 字节，前面填充 0
        //     let padding = vec![0u8; 32 - code.len()];
        //     code = [padding, code].concat();
        // }

        if self.memory.len() < (offset.clone() + BigUint::from(32u8)).to_usize().unwrap() {
            self.memory.resize(
                (offset.clone() + BigUint::from(32u8)).to_usize().unwrap(),
                0,
            ); // 将不足的部分填充为
        }

        // 将 32 字节数据写入内存中的偏移位置
        for (i, code_byte) in code.iter().enumerate() {
            self.memory[(mem_offset.clone() + BigUint::from(i)).to_usize().unwrap()] = *code_byte;
        }
        self.fill_memory();
    }
    fn extcodehash(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let addr_int = get_uint256(self.stack.pop().unwrap());
        let addr_str = format!("0x{}", vec_to_hex_string(addr_int.to_bytes_be()));
        let code = get_account_db().get_account(addr_str).code.clone();
        let code_hash = keccak256(&code).to_vec();
        self.stack.push((BigUint::from_bytes_be(&code_hash), 0u8));
    }
    fn extcodesize(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let addr_int = get_uint256(self.stack.pop().unwrap());
        let addr_str = format!("0x{}", vec_to_hex_string(addr_int.to_bytes_be()));
        let code = get_account_db().get_account(addr_str).code.clone();
        self.stack.push((BigUint::from(code.len()), 0u8));
    }
}

#[test]
fn test_balance() {
    let excute_codes = "739bbfed6889322e016e0a02ee459d306fc19545d831";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn test_extcodesize() {
    let excute_codes = "739bbfed6889322e016e0a02ee459d306fc19545d83B";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn test_extcodehash() {
    let excute_codes = "739bbfed6889322e016e0a02ee459d306fc19545d83F";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", vec_to_hex_string(evm_test.stack[0].0.to_bytes_be()));
}

#[test]
fn test_extcodecopy() {
    let excute_codes = "60045F5F739bbfed6889322e016e0a02ee459d306fc19545d83C";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", vec_to_hex_string(evm_test.memory));
}
