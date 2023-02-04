pub mod gov22;
pub use crate::traits::strategy::*;
use ink_storage::traits::SpreadAllocate;
use openbrush::traits::{AccountId, Storage};

#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default, Debug)]
///SC Data
pub struct Data<T: SpreadAllocate> {
    pub master_dao: AccountId,
    pub data: T,
}
