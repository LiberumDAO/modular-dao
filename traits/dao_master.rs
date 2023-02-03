use ink_env::AccountId;

use openbrush::traits::{Balance, String};
use scale::{Decode, Encode};

///the master dao smart contract
#[openbrush::trait_definition]
pub trait DaoMaster {
    ///Returns DAO's name
    #[ink(message)]
    fn get_name(&self) -> String;
    ///Adds strategy SC - the SC under `strategy_address` has to implement `modular_dao::traits::Strategy`
    #[ink(message)]
    fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(),DaoError>; //DaoError>;
    ///Adds proposal SC - the SC under `proposal_address` has to implement `modular_dao::traits::Proposal`
    #[ink(message)]
    fn add_proposal_type(&mut self, proposal_address: AccountId) -> Result<(),DaoError>; //DaoError>;
    ///Returns cumulative vote weight of a given address for all strategies
    #[ink(message)]
    fn get_vote_weight(&self, address: AccountId) -> Result<Balance,DaoError>; //DaoError>;
    ///Returns `true` if `address` voted in any pending proposal
    #[ink(message)]
    fn in_active_proposal(&self, address: AccountId) -> bool ;
}

#[openbrush::wrapper]
pub type DaoMasterRef = dyn DaoMaster;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DaoError {
    SomeError
}