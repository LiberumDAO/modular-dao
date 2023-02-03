use crate::traits::dao_master::DaoMasterRef;
pub use crate::traits::deposit::*;
use openbrush::contracts::traits::psp22::PSP22Ref;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Balance, Storage},
};

use ink_prelude::vec::Vec;

#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default, Debug)]
///Data of the Deposit SC
pub struct Data {
    pub dao_master: AccountId,
    pub token_address: AccountId,
    pub deposits: Mapping<AccountId, Balance>,
}
///Default implementation of `modular_dao::traits::Deposit` SC
impl<T: Storage<Data>> Deposit for T {
    ///Allows user to deposit some PSP22 token
    /// TODO: reentrancy
    default fn make_deposit(&self, amount: Balance, data: Vec<u8>) -> Result<(), DepositError> {
        match PSP22Ref::transfer(
            &self.data().token_address,
            Self::env().account_id(),
            amount,
            data,
        ) {
            Ok(()) => Ok(()),
            _ => Err(DepositError::SomeError),
        }
    }
    ///Allows user to withdraw their deposit token if user didn't vote in any still pending proposal
    /// TODO: reentrancy
    default fn withdraw_deposit(&self, amount: Balance, data: Vec<u8>) -> Result<(), DepositError> {
        if DaoMasterRef::in_active_proposal(&self.data().dao_master, Self::env().caller()) {
            return Err(DepositError::SomeError);
        }
        match PSP22Ref::transfer(
            &self.data().token_address,
            Self::env().caller(),
            amount,
            data,
        ) {
            Ok(()) => Ok(()),
            _ => Err(DepositError::SomeError),
        }
    }
    ///Returns the deposit amout of a given address
    default fn deposit_of(&self, account: AccountId) -> Result<Balance, DepositError> {
        self.data().deposits.get(&account).ok_or(DepositError::SomeError)
    }
}
