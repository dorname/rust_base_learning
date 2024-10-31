use crate::evm::Evm;
use crate::ops::traits::*;
use crate::utils::*;
use log::*;
use num_bigint::BigUint;
use num_traits::{zero, ToPrimitive};

use super::account;
impl Call for Evm {
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
        let mut db = get_account_db();
        let mut db_temp = get_account_db();
        let account_source = db.get_account_mut(self.txn.get_caller());

        // //获取目标账户
        let account_dest = db_temp.get_account_mut(self.txn.get_to());

        //判断调用账户是否有足够的资金
        if &account_source.balance < &value {
            self.success = false;
            info!("insufficient balance");
            self.stack.push((zero(), 0u8));
        }

        //更新余额
        account_source.balance -= value.clone();
        account_dest.balance += value.clone();

        if account_dest.code.len() > 0 {}
    }
    fn delegatecall(&mut self) {}
    fn staticcall(&mut self) {}
}
