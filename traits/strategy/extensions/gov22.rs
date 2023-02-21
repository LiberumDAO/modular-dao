use openbrush::traits::AccountId;
use scale::{Decode, Encode};
use openbrush::contracts::access_control::extensions::enumerable::*;

#[openbrush::trait_definition]
pub trait GOV22 {
    ///Returns c
    #[ink(message)]
    fn set_token_address(&mut self, token_address: AccountId) -> Result<(),Error>;
    #[ink(message)]
    fn get_token_address(&self) -> AccountId;
}
///Returns cumulative vote weight of a given address 
#[openbrush::wrapper]
pub type GOV22Ref = dyn GOV22;

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AccessControlError(AccessControlError),
}

impl From<AccessControlError> for Error {
    fn from(access: AccessControlError) -> Self {
        Error::AccessControlError(access)
    }
}