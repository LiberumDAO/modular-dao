use ink_env::AccountId;

use openbrush::traits::{Balance, String};
use scale::{Decode, Encode};

#[openbrush::trait_definition]
pub trait DaoMaster {

    #[ink(message)]
    fn get_name(&self) -> String;

    #[ink(message)]
    fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(),String>; //DaoError>;

    #[ink(message)]
    fn add_proposal_type(&mut self, proposal_address: AccountId) -> Result<(),String>; //DaoError>;

    #[ink(message)]
    fn get_vote_weight(&self, address: AccountId) -> Result<Balance,String>; //DaoError>;
}

#[openbrush::wrapper]
pub type DaoMasterRef = dyn DaoMaster;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DaoError {
    SomeError
}