use num_bigint::BigUint;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Account {
    pub balance: BigUint,
    pub nonce: BigUint,
    pub storage: HashMap<BigUint, (BigUint, u8)>,
    pub code: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct AccountDb {
    data: HashMap<String, Account>,
}

impl AccountDb {
    pub fn mock() -> Self {
        let account = Account {
            balance: BigUint::from(100u8),
            nonce: BigUint::from(1u8),
            storage: HashMap::new(),
            code: hex::decode("60006000").unwrap(),
        };
        let mut accounts: HashMap<String, Account> = HashMap::new();
        accounts.insert(
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            account,
        );
        Self { data: accounts }
    }
    pub fn get_account(&self, address: String) -> &Account {
        self.data.get(&address).unwrap()
    }
    pub fn get_account_mut(&mut self, address: String) -> &mut Account {
        self.data.get_mut(&address).unwrap()
    }
}
