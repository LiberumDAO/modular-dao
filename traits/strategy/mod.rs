pub mod extensions;
use openbrush::traits::AccountId;

///Strategy SC
#[openbrush::trait_definition]
pub trait Strategy {
    ///Returns c
    #[ink(message)]
    fn get_vote_weight(&self, voter_address: AccountId) -> Option<u128>;
}
///Returns cumulative vote weight of a given address 
#[openbrush::wrapper]
pub type StrategyRef = dyn Strategy;