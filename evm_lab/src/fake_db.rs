use std::collections::HashMap;
use num_bigint::BigUint;

pub struct ACCOUNT  {
    pub balance:BigUint,
    pub nonce:BigUint,
    pub storage:HashMap<BigUint, (BigUint, u8)>,
    pub code:Vec<u8>,
}

pub struct AccountDb {
    data:HashMap::<String,ACCOUNT>
}

impl AccountDb {
    pub fn mock()-> Self{
        let account = ACCOUNT {
            balance: BigUint::from(100u8), 
        nonce: BigUint::from(1u8), 
        storage: HashMap::new(),
        code: hex::decode("60006000").unwrap()  
        };
        let mut accounts:HashMap::<String,ACCOUNT> = HashMap::new();
        accounts.insert("0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(), account);
        Self {
            data:accounts
        }
    }
    pub fn get_account(&self,address:String)-> &ACCOUNT{
        self.data.get(&address).unwrap()
    }
}