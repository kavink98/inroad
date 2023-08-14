use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::{log, AccountId, Balance, Gas, Promise, PromiseError, PublicKey};

const NEAR_PER_STORAGE: Balance = 10_000_000_000_000_000_000; // 10e18yⓃ
const TGAS: Gas = Gas(10u64.pow(12));
const NO_DEPOSIT: Balance = 0;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractInitArgs {
    caller: AccountId
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MainContract {
    bidding_contracts: Vector<AccountId>,
    code: Vec<u8>,
    bidding_list: UnorderedMap<AccountId, bool>,
    testing_list: UnorderedMap<AccountId, bool>,
}

impl Default for MainContract {
    fn default() -> Self {
        Self {
            bidding_contracts: Vector::new(b"contract".to_vec()),
            code: include_bytes!("../build/tender_bidding.wasm").to_vec(),
            bidding_list: UnorderedMap::new(b"bid".to_vec()),
            testing_list: UnorderedMap::new(b"test".to_vec()),
        }
    }
}

#[near_bindgen]
impl MainContract {

    #[private]
    pub fn add_to_bidding_list(&mut self, account_id: AccountId) {
        self.bidding_list.insert(&account_id, &true);
    }

    #[private]
    pub fn add_to_testing_list(&mut self, account_id: AccountId) {
        self.testing_list.insert(&account_id, &true);
    }

    pub fn is_in_bidding_list(&self, account_id: AccountId) -> bool {
        self.bidding_list.get(&account_id).is_some()
    }

    pub fn is_in_testing_list(&self, account_id: AccountId) -> bool {
        self.testing_list.get(&account_id).is_some()
    }
}

#[near_bindgen]
impl MainContract {
    // Function to deploy a new bidding contract and store its contract ID
    #[payable]
    #[private]
    pub fn create_factory_subaccount_and_deploy(
        &mut self,
        project_name: String,
        public_key: Option<PublicKey>,
    ) -> Promise {
        // Assert the sub-account is valid
        let current_account = env::current_account_id();
        let current_account_name = current_account.to_string();
        let subaccount: AccountId = format!("{project_name}.{current_account_name}")
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
        assert!(
            attached >= minimum_needed,
            "Attach at least {} yⓃ",
            minimum_needed
        );
        let init_args = near_sdk::serde_json::to_vec(&ContractInitArgs { caller: current_account}).unwrap();
        let mut promise = Promise::new(subaccount.clone())
            .create_account()
            .transfer(attached)
            .deploy_contract(code)
            .function_call("init".to_owned(), init_args, NO_DEPOSIT, TGAS * 5);

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
        #[callback_result] create_deploy_result: Result<(), PromiseError>,
    ) -> bool {
        if let Ok(_result) = create_deploy_result {
            log!(format!("Correctly created and deployed to {account}"));
            return true;
        };

        log!(format!(
            "Error creating {account}, returning {attached}yⓃ to {user}"
        ));
        Promise::new(user).transfer(attached);
        false
    }
}
