use crate::const_var::*;
use log::*;
use num_bigint::BigUint;

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
