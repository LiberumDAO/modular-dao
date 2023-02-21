use openbrush::traits::AccountId;
use scale::{Decode, Encode};
use ink::prelude::vec::Vec;
use openbrush::contracts::access_control::extensions::enumerable::*;
///Strategy SC
#[openbrush::trait_definition]
pub trait Whitelist {
    ///Returns c
    #[ink(message)]
    fn add_member(&mut self, voter_address: AccountId) -> Result<(),Error>;
    #[ink(message)]
    fn remove_member(&mut self, voter_address: AccountId) -> Result<(),Error>;
    #[ink(message)]
    fn get_members(&self) -> Vec<AccountId>;
    #[ink(message)]
    fn is_member(&self, voter_address: AccountId) -> bool;
}
///Returns cumulative vote weight of a given address 
#[openbrush::wrapper]
pub type WhitelistRef = dyn Whitelist;

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AccessControlError(AccessControlError),
    SomeError,
}

impl From<AccessControlError> for Error {
    fn from(access: AccessControlError) -> Self {
        Error::AccessControlError(access)
    }
}