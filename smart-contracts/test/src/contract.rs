use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, ext_contract, near_bindgen};
use near_sdk::{log, AccountId, Gas};

macro_rules! check_valid_role{
    ($contract_id:expr,$is_in_list:expr, $func:ident ( $($args:expr),* )) => {
            ext_contract::ext($contract_id)
                .is_in_bidding_list(env::signer_account_id())
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(XCC_GAS)
                        . $func($($args),*),
                );
    };
}

pub const XCC_GAS: Gas = Gas(20_000_000_000_000);

#[ext_contract(ext_contract)]
pub trait MainContract {
    fn is_in_testing_list(&self, account_id: AccountId) -> bool;
    fn is_in_bidding_list(&self, account_id: AccountId) -> bool;
}
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Args {
    account_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MyContract {}

impl Default for MyContract {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl MyContract {
    /*
        Multiple different functions here
        fn1
        fn2
        ....
    */

    pub fn call_external_function_one(&mut self) {
        let account_id: AccountId = "test.kavin.testnet".to_string().parse().unwrap();
        let mut my_map: HashMap<i32, String> = HashMap::new();

        my_map.insert(42, String::from("Hello"));
        my_map.insert(99, String::from("World"));
        check_valid_role!(account_id, is_in_bidding_list, get_external_call_result(my_map));
    }

    #[private]
    pub fn get_external_call_result(
        &self,
        #[callback_result] is_allowed: Result<bool, near_sdk::PromiseError>,
        my_map: HashMap<i32, String>,
    ) -> bool {
        if let Ok(true) = is_allowed {
            log!("It works, I'm a genius!");
    
            // Print values from the HashMap
            for (key, value) in &my_map {
                log!("Key: {}, Value: {}", key, value);
            }
    
            true
        } else {
            panic!("Something");
        }
    }
}
