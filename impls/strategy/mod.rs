pub mod extensions;
pub use crate::traits::strategy::*;
use openbrush::traits::{AccountId, Storage};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
///SC Data
pub struct Data {
    pub master_dao: AccountId,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            master_dao: [0u8; 32].into(),
        }
    }
}

impl<T: Storage<Data>> Strategy for T {
    default fn get_vote_weight(&self, _address: AccountId) -> Option<u128> {
        // the logic could include getting some values from MasterDao contract
        // checking balance of a particular token of the `address`
        // basically, determines the "logic" of the strategy
        Some(1)
    }
}