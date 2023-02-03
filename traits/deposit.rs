use ink_env::AccountId;

use openbrush::{traits::Balance, contracts::psp22::PSP22Error};
use scale::{Decode, Encode};

#[openbrush::trait_definition]
pub trait Deposit {

    #[ink(message)]
    fn make_deposit(&self, amount: Balance, data: Vec<u8>) -> Result<(),DepositError>;

    #[ink(message)]
    fn withdraw_deposit(&self, amount: Balance,data: Vec<u8>) -> Result<(),DepositError>;

    #[ink(message)]
    fn deposit_of(&self, account: AccountId) -> Result<Balance,DepositError>;

}

#[openbrush::wrapper]
pub type DepositRef = dyn Deposit;

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DepositError {
    PSP22Error(PSP22Error),
    SomeError
}

impl From<PSP22Error> for DepositError {
    fn from(error: PSP22Error) -> Self {
        DepositError::SomeError
    }
}