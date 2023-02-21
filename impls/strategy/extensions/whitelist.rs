use crate::impls::dao::FOUNDER;
use crate::impls::strategy;
use crate::traits::dao;
pub use crate::traits::strategy::extensions::whitelist::*;
use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, Storage};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
///SC Data
pub struct Data {
    pub list: Vec<AccountId>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            list: Default::default(),
        }
    }
}

impl<T: Storage<Data> + Storage<strategy::Data>> Whitelist for T {
    default fn add_member(&mut self, voter_address: AccountId) -> Result<(), Error> {
        if dao::DaoContractRef::has_role(
            &self.data::<strategy::Data>().master_dao,
            FOUNDER,
            Self::env().caller(),
        ) {
            if self.data::<Data>().list.contains(&voter_address) {
                return Err(Error::SomeError);
            }
            self.data::<Data>().list.push(voter_address);
            return Ok(());
        }
        return Err(Error::AccessControlError(
            openbrush::contracts::access_control::AccessControlError::MissingRole,
        ));
    }

    default fn remove_member(&mut self, voter_address: AccountId) -> Result<(), Error> {
        if dao::DaoContractRef::has_role(
            &self.data::<strategy::Data>().master_dao,
            FOUNDER,
            Self::env().caller(),
        ) {
            if self.data::<Data>().list.contains(&voter_address) {
                let i = self
                    .data::<Data>()
                    .list
                    .iter()
                    .position(|&r| r == voter_address)
                    .unwrap();
                self.data::<Data>().list.remove(i);
                return Ok(());
            }
        }
        return Err(Error::AccessControlError(
            openbrush::contracts::access_control::AccessControlError::MissingRole,
        ));
    }

    default fn get_members(&self) -> Vec<AccountId> {
        self.data::<Data>().list.clone()
    }

    default fn is_member(&self, voter_address: AccountId) -> bool {
        self.data::<Data>().list.contains(&voter_address)
    }
}
