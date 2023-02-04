pub mod role_dao;

use ink_env::AccountId;

use openbrush::contracts::access_control::*;
use scale::{Decode, Encode};

///the master dao smart contract
#[openbrush::trait_definition]
pub trait Dao {
    ///Adds strategy SC - the SC under `strategy_address` has to implement `modular_dao::traits::Strategy`
    #[ink(message)]
    fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(), Error>;
    ///Adds proposal SC - the SC under `proposal_address` has to implement `modular_dao::traits::Proposal`
    #[ink(message)]
    fn add_proposal_type(&mut self, proposal_address: AccountId) -> Result<(), Error>;
    ///Returns cumulative vote weight of a given address for all strategies
    #[ink(message)]
    fn get_vote_weight(&self, address: AccountId) -> Result<u128, Error>;
    ///Returns `true` if `address` voted in any pending proposal
    #[ink(message)]
    fn in_active_proposal(&self, address: AccountId) -> bool;
}

#[openbrush::wrapper]
pub type DaoRef = dyn Dao + AccessControl;

///TODO appropriate errors
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AccessControlError(AccessControlError),
    StrategyAlreadyIncorporated,
    ProposalTypeAlreadyIncorporated,
}

impl From<AccessControlError> for Error {
    fn from(access: AccessControlError) -> Self {
        Error::AccessControlError(access)
    }
}


