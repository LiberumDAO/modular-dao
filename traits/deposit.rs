use ink_env::AccountId;

use openbrush::traits::{Balance, String};
use scale::{Decode, Encode};

#[openbrush::trait_definition]
pub trait Deposit {

    #[ink(message)]
    fn make_deposit(&self, amount: Balance) -> Result<(),String>;

    #[ink(message)]
    fn withdraw_deposit(&self, amount: Balance) -> Result<(),String>;

    #[ink(message)]
    fn get_deposit(&self, account: AccountId) -> Result<Balance,String>;

}

#[openbrush::wrapper]
pub type DepositRef = dyn Deposit;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DepositError {
    SomeError
}