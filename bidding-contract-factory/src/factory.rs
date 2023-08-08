use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Promise, PublicKey, Gas, log};

const NEAR_PER_STORAGE: Balance = 10_000_000_000_000_000_000; // 10e18yⓃ
const TGAS: Gas = Gas(10u64.pow(12)); // 10e12yⓃ
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BiddingContractFactory {
    bidding_contracts: Vector<AccountId>,
    code: Vec<u8>,
}

impl Default for BiddingContractFactory {
    fn default() -> Self {
        Self {
            bidding_contracts: Vector::new(b"contract".to_vec()),
            code: include_bytes!("../build/tender_bidding.wasm").to_vec(),
        }
    }
}

#[near_bindgen]
impl BiddingContractFactory {
    // Function to deploy a new bidding contract and store its contract ID
    #[payable]
    pub fn create_factory_subaccount_and_deploy(
        &mut self,
        name: String,
        public_key: Option<PublicKey>,
    ) -> Promise {
        // Assert the sub-account is valid
        let current_account = env::current_account_id();
        let current_account_name = current_account.to_string();
        let subaccount: AccountId = format!("{name}.{current_account_name}", name = name)
            .parse()
            .unwrap();
        assert!(
            env::is_valid_account_id(subaccount.as_bytes()),
            "Invalid subaccount"
        );

        // Assert enough money is attached to create the account and deploy the contract
        let attached = env::attached_deposit();
        let code = self.code.clone();
        let contract_bytes = code.len() as u128;
        let minimum_needed = NEAR_PER_STORAGE * contract_bytes;
        assert!(attached >= minimum_needed, "Attach at least {} yⓃ", minimum_needed);
        let init_args = current_account.try_to_vec().unwrap();
        let mut promise = Promise::new(subaccount.clone())
            .create_account()
            .transfer(attached)
            .deploy_contract(code);

        // Add full access key if the user passes one
        if let Some(pk) = public_key {
            promise = promise.add_full_access_key(pk);
        }

        // Add callback
        promise.then(
            Self::ext(env::current_account_id()).create_factory_subaccount_and_deploy_callback(
                subaccount,
                env::predecessor_account_id(),
                attached,
            ),
        )
    }

    #[private]
    pub fn create_factory_subaccount_and_deploy_callback(
        &mut self,
        account: AccountId,
        user: AccountId,
        attached: Balance,
    ) -> bool {
        // Handle the promise callback, e.g., log or handle errors
        if env::promise_results_count() == 0 {
            // Success
            log!("Successfully created and deployed to {}", account);
        } else {
            // Error
            log!("Error creating {}", account);
            // Refund the attached deposit to the user
            Promise::new(user).transfer(attached);
        }
        true
    }
}
