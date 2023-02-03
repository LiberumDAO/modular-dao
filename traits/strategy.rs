use ink_env::AccountId;
use openbrush::traits::Balance;
use scale::{Decode, Encode};


#[openbrush::trait_definition]
pub trait Strategy {
    #[ink(message)]
    fn get_vote_weight(&self, voter_address: AccountId) -> Result<Balance,StrategyError>;//StrategyError>;
}

#[openbrush::wrapper]
pub type StrategyRef = dyn Strategy;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StrategyError {
    SomeError
}