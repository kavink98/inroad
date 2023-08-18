use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, PromiseError};

macro_rules! check_valid_role{
    ($owner:expr, $is_in_list:ident, $callback:ident ( $($args:expr),* )) => {
            main_contract::ext($owner.clone())
                .$is_in_list(env::signer_account_id())
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(XCC_GAS)
                        . $callback($($args),*),
                );
    };
}

macro_rules! assert_contract_state {
    ($state:expr, $expected:path, $message:expr) => {
        assert!(
            matches!($state, $expected),
            concat!("Invalid contract state: ", $message)
        );
    };
}

macro_rules! assert_permitted_role {
    ($self:expr, $func_name:ident ()) => {
        assert!(
            $self.$func_name(),
            "You do not have permissions to perform this task"
        );
    };
}

pub const XCC_GAS: Gas = Gas(20_000_000_000_000);

#[ext_contract(main_contract)]
pub trait MainContract {
    fn is_in_testing_list(&self, account_id: AccountId) -> bool;
    fn is_in_bidding_list(&self, account_id: AccountId) -> bool;
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    bidder_name: String,
    price: u64,
    database_hash: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Milestone {
    description: String,
    due_date: u64,
    completion_date: Option<u64>, // None if not completed, Some(date) if completed
    database_hash: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Test {
    description: String,
    success: bool,
    database_hash: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Comment {
    commenter: AccountId,
    thumbs_up: bool,
    message: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum ContractState {
    Disabled,
    Survey,
    Bidding,
    Selected,
    Construction,
    Inspection,
    PreCompleted,
    Testing,
    Completed,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BiddingContract {
    bids: UnorderedMap<AccountId, Bid>,
    winning_bidder: Option<AccountId>,
    owner: AccountId,
    state: ContractState,
    comments: Vector<Comment>,
    milestones: UnorderedMap<String, Milestone>,
    tests: UnorderedMap<String, Test>,
}

impl Default for BiddingContract {
    fn default() -> Self {
        Self {
            bids: UnorderedMap::new(b"bids".to_vec()),
            winning_bidder: None,
            owner: env::current_account_id(),
            state: ContractState::Survey,
            comments: Vector::new(b"comments".to_vec()),
            milestones: UnorderedMap::new(b"milestone".to_vec()),
            tests: UnorderedMap::new(b"test".to_vec()),
        }
    }
}

#[near_bindgen]
impl BiddingContract {
    #[init]
    pub fn init(caller: AccountId) -> Self {
        Self {
            bids: UnorderedMap::new(b"bids".to_vec()),
            winning_bidder: None,
            owner: caller,
            state: ContractState::Survey,
            comments: Vector::new(b"comments".to_vec()),
            milestones: UnorderedMap::new(b"milestone".to_vec()),
            tests: UnorderedMap::new(b"test".to_vec()),
        }
    }

    pub fn view_owner(&self) -> &AccountId {
        &self.owner
    }

    // Function to place a bid
    pub fn place_bid(&mut self, price: u64, bidder_name: String, database_hash: String) {
        check_valid_role!(
            self.owner,
            is_in_bidding_list,
            place_bid_callback(price, bidder_name, database_hash)
        );
    }

    #[private]
    pub fn place_bid_callback(
        &mut self,
        #[callback_result] is_allowed: Result<bool, PromiseError>,
        price: u64,
        bidder_name: String,
        database_hash: String,
    ) {
        if let Ok(true) = is_allowed {
            assert_contract_state!(
                self.state,
                ContractState::Bidding,
                "Contract is not open for bidding"
            );
            let bidder = env::signer_account_id();
            let new_bid = Bid {
                bidder_name,
                price,
                database_hash,
            };
            self.bids.insert(&bidder, &new_bid);
        } else {
            env::panic_str("Not allowed to place bids");
        }
    }

    // Function to view all the placed bids
    pub fn view_bids(&self) -> Vec<(AccountId, Bid)> {
        self.bids.to_vec()
    }

    // Function to get a specific bid by bidder's account ID
    pub fn get_bid_by_bidder(&self, bidder: AccountId) -> Option<Bid> {
        self.bids.get(&bidder)
    }

    pub fn choose_winner(
        &mut self,
        bidder: AccountId,
        milestones: HashMap<String, Milestone>,
        _tests: HashMap<String, Test>,
    ) {
        assert_contract_state!(
            self.state,
            ContractState::Bidding,
            "No tender exists yet or tender has been closed"
        );
        assert_permitted_role!(self, only_owner());
        self.winning_bidder = Some(bidder);
        self.insert_map_into_unordered_map(milestones);
        self.state = ContractState::Selected;
    }

    pub fn get_winner(&self) -> Option<Bid> {
        match &self.winning_bidder {
            Some(bidder) => self.bids.get(bidder),
            None => None,
        }
    }
}

#[near_bindgen]
impl BiddingContract {
    // Function to get the public comments for the project
    pub fn place_comments(&mut self, thumbs_up: bool, message: String) {
        assert!(
            matches!(self.state, ContractState::Survey),
            "This project is no longer accepting public survey"
        );
        let comment = Comment {
            commenter: env::signer_account_id(),
            thumbs_up,
            message,
        };
        self.comments.push(&comment);
    }

    pub fn view_comments(&self) -> Vec<Comment> {
        self.comments.to_vec()
    }
}

#[near_bindgen]
impl BiddingContract {
    pub fn set_state_to_disabled(&mut self) {
        assert_permitted_role!(self, only_owner());
        self.state = ContractState::Disabled;
    }

    pub fn set_state_to_bid(&mut self) {
        assert_contract_state!(
            self.state,
            ContractState::Survey,
            "State has to be survey in order to move to bidding"
        );
        assert_permitted_role!(self, only_owner());
        self.state = ContractState::Bidding;
    }

    pub fn set_state_to_construction(&mut self) {
        assert_contract_state!(
            self.state,
            ContractState::Selected,
            "State has to be selected in order to move to construction"
        );
        assert_permitted_role!(self, only_owner());
        self.state = ContractState::Construction
    }

    pub fn set_state_to_precompleted(&mut self) {
        assert_contract_state!(
            self.state,
            ContractState::Construction,
            "State has to be in construction in order to move to precompleted"
        );
        assert_permitted_role!(self, only_winner());
        self.state = ContractState::PreCompleted;
    }

    pub fn set_state_to_testing(&mut self) {
        assert_contract_state!(
            self.state,
            ContractState::PreCompleted,
            "State has to be in precompleted in order to move to completed"
        );
        assert_permitted_role!(self, only_owner());
        self.state = ContractState::Testing;
    }
}

#[near_bindgen]
impl BiddingContract {
    pub fn update_milestone(&mut self, name: String, database_hash: String) {
        assert_permitted_role!(self, only_winner());

        let mut milestone = self.milestones.get(&name).expect("Milestone not found");
        assert!(
            milestone.database_hash.is_none() && milestone.completion_date.is_none(),
            "Milestone already achieved"
        );

        let current_block_timestamp = env::block_timestamp();

        milestone.completion_date = Some(current_block_timestamp);
        milestone.database_hash = Some(database_hash);

        self.milestones.insert(&name, &milestone);
    }

    pub fn view_milestones(&self) -> Vec<(String, Milestone)> {
        self.milestones.to_vec()
    }
}

#[near_bindgen]
impl BiddingContract {
    #[private]
    pub fn insert_map_into_unordered_map(&mut self, map_arg: HashMap<String, Milestone>) {
        for (key, value) in map_arg {
            self.milestones.insert(&key, &value);
        }
    }

    #[private]
    pub fn only_owner(&self) -> bool {
        env::signer_account_id() == self.owner
    }

    #[private]
    pub fn only_winner(&self) -> bool {
        if let Some(winning_bidder) = &self.winning_bidder {
            return *winning_bidder == env::signer_account_id();
        }
        false
    }
}
