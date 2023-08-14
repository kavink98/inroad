use std::collections::HashMap;

use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::serde::{Serialize, Deserialize};

macro_rules! assert_contract_state {
    ($state:expr, $expected:expr) => {
        assert!(
            matches!($state, $expected),
            concat!("Invalid contract state: ", stringify!($expected))
        );
    };
}

macro_rules! assert_permitted_role {
    ($self:expr, $func_name:ident ()) => {
        assert!($self.$func_name(), "You do not have permissions to perform this task");
    };
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
    database_hash: Option<String>
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Comment {
    commenter: AccountId,
    thumbs_up: bool,
    message: String
}

#[derive(BorshDeserialize, BorshSerialize)]
enum ContractState {
    Bidding,
    Survey,
    Disabled,
    Construction,
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
    milestones: UnorderedMap<String, Milestone>
}

impl Default for BiddingContract {
    fn default() -> Self {
        Self {
            bids: UnorderedMap::new(b"bids".to_vec()),
            winning_bidder: None,
            owner: env::current_account_id(),
            state: ContractState::Survey,
            comments: Vector::new(b"comments".to_vec()),    
            milestones: UnorderedMap::new(b"milestone".to_vec())       
        }
    }
}

#[near_bindgen]
impl BiddingContract {

    #[private]
    #[init]
    pub fn init(caller: AccountId) -> Self {
        Self {
            bids: UnorderedMap::new(b"bids".to_vec()),
            winning_bidder: None,
            owner: caller,
            state: ContractState::Survey,
            comments: Vector::new(b"comments".to_vec()),    
            milestones: UnorderedMap::new(b"milestone".to_vec())       
        }
    }

    // Function to place a bid
    pub fn place_bid(&mut self, price: u64, bidder_name: String, database_hash: String) {
        assert!(matches!(self.state, ContractState::Bidding), "Not open for bids currently");
        let bidder = env::signer_account_id();
        let new_bid = Bid {
            bidder_name,
            price,
            database_hash,
        };
        self.bids.insert(&bidder, &new_bid);
    }

    // Function to view all the placed bids
    pub fn view_bids(&self) -> Vec<(AccountId, Bid)> {
        self.bids.to_vec()
    }

    // Function to get a specific bid by bidder's account ID
    pub fn get_bid_by_bidder(&self, bidder: AccountId) -> Option<Bid> {
        self.bids.get(&bidder)
    }

    pub fn choose_winner(&mut self, bidder: AccountId, milestones: HashMap<String, Milestone>) {
        assert!(matches!(self.state, ContractState::Bidding), "No tender exists yet or tender has been closed");
        assert_permitted_role!(self, only_owner());
        self.winning_bidder = Some(bidder);
        self.insert_map_into_unordered_map(milestones);
        self.state = ContractState::Construction;
    }
 
    pub fn get_winner(&self) -> Option<Bid> {
        match &self.winning_bidder {
            Some(bidder) => self.bids.get(bidder),
            None => None,
        }
    }

    // Function to get the public comments for the project
    pub fn place_comments(&mut self, thumbs_up: bool, message: String) {
        assert!(matches!(self.state, ContractState::Survey), "This project is no longer accepting public survey");
        let comment = Comment {
            commenter: env::signer_account_id(),
            thumbs_up,
            message
        }; 
        self.comments.push(&comment);
    }

    pub fn view_comments(&self) -> Vec<Comment> {
        self.comments.to_vec()
    }

    pub fn set_state_to_bid(&mut self) {
        assert_permitted_role!(self, only_owner());
        self.state = ContractState::Bidding; 
    }

    pub fn set_state_to_disabled(&mut self) {
        assert_permitted_role!(self, only_owner());
        self.state = ContractState::Disabled; 
    }

    pub fn update_milestone(&mut self, name: String, database_hash: String) {
        assert_permitted_role!(self, only_winner());

        let mut milestone = self.milestones.get(&name).expect("Milestone not found");
        assert!(milestone.database_hash.is_some(), "Milestone already achieved");

        let current_block_timestamp = env::block_timestamp();

        milestone.completion_date = Some(current_block_timestamp);
        milestone.database_hash = Some(database_hash);

        self.milestones.insert(&name, &milestone);
    }

    pub fn view_milestones(&self) -> Vec<(String, Milestone)> {
        self.milestones.to_vec()
    }

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
