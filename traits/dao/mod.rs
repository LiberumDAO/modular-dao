use openbrush::contracts::access_control::*;
use openbrush::traits::AccountId;
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
    fn get_vote_weight(&self, address: AccountId) -> Result<Option<u128>, Error>;
    ///Returns `true` if `address` voted in any pending proposal
    #[ink(message)]
    fn in_active_proposal(&self, address: AccountId) -> bool;
    #[ink(message)]
    fn grant_role_in_dao(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError>;
    #[ink(message)]
    fn revoke_role_in_dao(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError>;
    #[ink(message)]
    fn private_voting_allowed(&self) -> bool;
    #[ink(message)]
    fn liberum_veto_allowed(&self) -> bool;
    #[ink(message)]
    fn delegate_vote_allowed(&self) -> bool;
    #[ink(message)]
    fn delegate_vote(&mut self, to_account: AccountId) -> Result<(), Error>;
    #[ink(message)]
    fn revoke_delegate_vote(&mut self) -> Result<(), Error>;
    #[ink(message)]
    fn has_delegated(&mut self, account: AccountId) -> bool;
    
}

#[openbrush::wrapper]
pub type DaoContractRef = dyn Dao + AccessControl;

///TODO appropriate errors
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AccessControlError(AccessControlError),
    StrategyAlreadyIncorporated,
    ProposalTypeAlreadyIncorporated,
    VoteAlreadyDelegated,
    VoteNotDelegated,
    //SomeError,
}

impl From<AccessControlError> for Error {
    fn from(access: AccessControlError) -> Self {
        Error::AccessControlError(access)
    }
}
