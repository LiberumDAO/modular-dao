use crate::impls::dao::FOUNDER;
use crate::impls::strategy;
use crate::traits::dao;
pub use crate::traits::strategy::extensions::gov22::*;
use openbrush::traits::{AccountId, Storage};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
///SC Data
pub struct Data {
    pub token_address: AccountId,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            token_address: [0u8; 32].into(),
        }
    }
}

impl<T: Storage<Data> + Storage<strategy::Data>> GOV22 for T {
    default fn set_token_address(&mut self, token_address: AccountId) -> Result<(),Error> {
        if dao::DaoContractRef::has_role(
            &self.data::<strategy::Data>().master_dao,
            FOUNDER,
            Self::env().caller(),
        ) {
            self.data::<Data>().token_address = token_address;
            return Ok(());
        }
        return Err(Error::AccessControlError(
            openbrush::contracts::access_control::AccessControlError::MissingRole,
        ));
    }

    default fn get_token_address(&self) -> AccountId {
        self.data::<Data>().token_address
    }
}
