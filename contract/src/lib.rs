use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    bidder_name: String,
    price: u64, // Assuming the price is an integer
    database_hash: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BiddingContract {
    bids: UnorderedMap<AccountId, Bid>,
    winning_bidder: Option<AccountId>,
}

impl Default for BiddingContract {
    fn default() -> Self {
        Self {
            bids: UnorderedMap::new(b"b".to_vec()),
            winning_bidder: None,
        }
    }
}

#[near_bindgen]
impl BiddingContract {
    // Function to place a bid
    pub fn place_bid(&mut self, price: u64, bidder_name: String, database_hash: String) {
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

    // Function to choose the winning bidder
    pub fn choose_winner(&mut self, bidder: AccountId) {
        self.winning_bidder = Some(bidder);
    }

    // Function to get the winning bidder
    pub fn get_winner(&self) -> Option<Bid> {
        match &self.winning_bidder {
            Some(bidder) => self.bids.get(bidder),
            None => None,
        }
    }
}
