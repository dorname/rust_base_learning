use crate::utils::*;
use log::*;
use num_bigint::BigUint;
pub struct LogTemplate {
    //操作符名称
    op_name: String,
    //获取运算符符号
    op: String,
    //数值a
    unit_a: (BigUint, u8),
    //数值b
    unit_b: (BigUint, u8),
    //数值c
    unit_c: (BigUint, u8),
    //计算结果
    result: BigUint,
    //结果是否为负数
    is_negative: u8,
}
impl LogTemplate {
    pub fn log_two_cal(&self) {
        let a = get_uint256(self.unit_a.clone());
        let b = get_uint256(self.unit_b.clone());
        let sign_a = if self.unit_a.1 != 0 { "-" } else { "" };
        let sign_b = if self.unit_b.1 != 0 { "-" } else { "" };
        info!(
            "{}计算过程:({}{:?}{}{}{:?})",
            self.op_name, sign_a, a, self.op, sign_b, b
        );
    }

    pub fn log_three_cal(&self) {
        let a = get_uint256(self.unit_a.clone());
        let b = get_uint256(self.unit_b.clone());
        let c = get_uint256(self.unit_c.clone());
        let sign_a = if self.unit_a.1 != 0 { "-" } else { "" };
        let sign_b = if self.unit_b.1 != 0 { "-" } else { "" };
        let sign_c = if self.unit_c.1 != 0 { "-" } else { "" };
        info!(
            "{}计算过程:({}{:?}{}{}{:?}%{}{})",
            self.op_name, sign_a, a, self.op, sign_b, b, sign_c, c
        );
    }

    pub fn log_store_val(&self) {
        info!(
            "{}的存储值:{:?}",
            self.op_name,
            vec_to_hex_string(self.result.to_radix_be(16))
        );
    }

    pub fn log_real_val(&self) {
        let sign_result = if self.is_negative == 1 { "-" } else { "" };
        info!(
            "{}的真实值:{}{:?}",
            self.op_name,
            sign_result,
            get_uint256((self.result.clone(), self.is_negative))
        );
    }

    pub fn set_result(&mut self, result: BigUint) {
        self.result = result;
    }

    pub fn set_is_negative(&mut self, is_negative: u8) {
        self.is_negative = is_negative;
    }

    pub fn new_two_cal(
        op_name: String,
        op: String,
        unit_a: (BigUint, u8),
        unit_b: (BigUint, u8),
    ) -> Self {
        Self {
            op_name,
            op,
            unit_a,
            unit_b,
            unit_c: (BigUint::from(0u8), 0),
            result: BigUint::from(0u8),
            is_negative: 0u8,
        }
    }

    pub fn new_three_cal(
        op_name: String,
        op: String,
        unit_a: (BigUint, u8),
        unit_b: (BigUint, u8),
        unit_c: (BigUint, u8),
    ) -> Self {
        Self {
            op_name,
            op,
            unit_a,
            unit_b,
            unit_c,
            result: BigUint::from(0u8),
            is_negative: 0u8,
        }
    }
}
pub fn log_mul_mod(a: BigUint, b: BigUint, c: BigUint, result: BigUint, is_negative: bool) {
    if is_negative {
        info!("MUL_MODk计算过程:({:?}*{:?})%(-{:?})", a, b, c);
        info!(
            "MUL_MOD的存储值:{:?}",
            vec_to_hex_string(result.to_radix_be(16))
        );
        info!("MUL_MOD的真实值:-{:?}", get_uint256((result.clone(), 1u8)));
    } else {
        info!("MUL_MODk计算过程:({:?}*{:?})%{:?}", a, b, c);
        info!("MUL_MOD:{:?}", vec_to_hex_string(result.to_radix_be(16)));
    }
}

pub fn log_mul(a: BigUint, b: BigUint, result: BigUint, is_negative: bool) {
    if is_negative {
        info!("MUL计算过程:({:?}*{:?})", a, b);
        info!(
            "MUL的存储值:{:?}",
            vec_to_hex_string(result.to_radix_be(16))
        );
        info!("MUL的真实值:-{:?}", get_uint256((result.clone(), 1u8)));
    } else {
        info!("MUL计算过程:({:?}*{:?})", a, b);
        info!("MUL:{:?}", vec_to_hex_string(result.to_radix_be(16)));
    }
}

pub fn log_add(sign_a: u8, sign_b: u8, a: BigUint, b: BigUint, result: BigUint, is_negative: bool) {
    let s_a = if sign_a != 0 { "-" } else { "" };
    let s_b = if sign_b != 0 { "-" } else { "" };
    if is_negative {
        info!("ADD计算过程:({}{:?}+{}{:?})", s_a, a, s_b, b);
        info!(
            "ADD的存储值:{:?}",
            vec_to_hex_string(result.to_radix_be(16))
        );
        info!("ADD的真实值:-{:?}", get_uint256((result.clone(), 1u8)));
    } else {
        info!("ADD计算过程:({}{:?}+{}{:?})", s_a, a, s_b, b);
        info!("ADD:{:?}", vec_to_hex_string(result.to_radix_be(16)));
    }
}
