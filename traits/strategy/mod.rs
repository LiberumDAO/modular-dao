use openbrush::traits::AccountId;
use openbrush::traits::Balance;
use scale::{Decode, Encode};

///Strategy SC
#[openbrush::trait_definition]
pub trait Strategy {
    ///Returns c
    #[ink(message)]
    fn get_vote_weight(&self, voter_address: AccountId) -> Result<Balance,Error>;
}
///Returns cumulative vote weight of a given address 
#[openbrush::wrapper]
pub type StrategyRef = dyn Strategy;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    SomeError
}