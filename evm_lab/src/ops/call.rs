use std::f32::consts::E;

use crate::ops::traits::*;
use crate::utils::*;
use crate::{evm::Evm, transaction::Transaction};
use log::*;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{zero, ToPrimitive};

impl Call for Evm {
    /// call指令
    /// ```
    /// let excute_codes = "6001601f5f5f6001731000000000000000000000000000000000000c425ff15f51";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn call(&mut self) {
        if self.stack.len() < 7 {
            panic!("stack underflow");
        }
        let gas = get_uint256(self.stack.pop().unwrap());
        let to = get_uint256(self.stack.pop().unwrap());
        let value = get_uint256(self.stack.pop().unwrap());
        let mem_in_offset = get_uint256(self.stack.pop().unwrap());
        let mem_in_size = get_uint256(self.stack.pop().unwrap());
        let mem_out_offset = get_uint256(self.stack.pop().unwrap());
        let mem_out_size = get_uint256(self.stack.pop().unwrap());

        // 拓展内存
        if self.memory.len() < &mem_in_offset.to_usize().unwrap() + &mem_in_size.to_usize().unwrap()
        {
            self.memory.resize(
                &mem_in_offset.to_usize().unwrap() + &mem_in_size.to_usize().unwrap(),
                0u8,
            );
        }

        let data = self.memory[mem_in_offset.to_usize().unwrap()
            ..mem_in_offset.to_usize().unwrap() + mem_in_size.to_usize().unwrap()]
            .to_vec();

        //获取调用账户
        let mut db = get_account_db_for_calltest();
        let mut db_temp = get_account_db_for_calltest();
        info!("caller:{}", self.txn.get_caller());
        info!("to:{}", format!("0x{}", hex::encode(to.to_bytes_be())));
        let account_source = db.get_account_mut(self.txn.get_caller());

        // //获取目标账户
        let account_dest = db_temp.get_account_mut(format!("0x{}", hex::encode(to.to_bytes_be())));

        //判断调用账户是否有足够的资金
        if &account_source.balance < &value {
            self.success = false;
            info!("insufficient balance");
            self.stack.push((zero(), 0u8));
        }

        //更新余额
        account_source.balance -= value.clone();
        account_dest.balance += value.clone();

        //构建上下文
        let txn = Transaction::init(
            zero(),
            self.txn.get_gas_price().clone(),
            self.txn.get_gas_limit().clone(),
            hex::encode(to.to_bytes_be()),
            value,
            hex::encode(data),
            self.txn.get_caller(),
            self.txn.get_origin(),
            hex::encode(to.to_bytes_be()),
            zero(),
            zero(),
            zero(),
        );

        // 初始化子EVM执行环境
        let mut evm_sub = Evm::init_evm(account_dest.code.clone(), txn);
        evm_sub.run();

        // 拓展内存
        let out_len = (&mem_out_offset + &mem_out_size).to_usize().unwrap();
        if self.memory.len() < out_len {
            self.memory.resize(out_len, 0u8);
        }

        self.memory[mem_out_offset.to_usize().unwrap()..out_len]
            .copy_from_slice(&evm_sub.return_data[0..]);

        if evm_sub.success {
            self.stack.push((BigUint::from(1u8), 0u8));
        } else {
            self.stack.push((zero(), 0u8));
        }
    }
    fn delegatecall(&mut self) {}
    fn staticcall(&mut self) {}
}

#[test]
fn test_call() {
    let excute_codes = "6001601f5f5f6001731000000000000000000000000000000000000c425ff15f51";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}
