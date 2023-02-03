pub use crate::traits::dao_master::*;
pub use crate::traits::strategy::*;
use openbrush::traits::{AccountId, Balance, Storage};

#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default, Debug)]
///SC Data
pub struct Data {
    pub master_dao: AccountId,
}
///Default implementation of the `modular_dao::traits::DaoMaster`
impl<T: Storage<Data>> Strategy for T {
    default fn get_vote_weight(&self, address: AccountId) -> Result<Balance, StrategyError> {
        // the logic could include getting some values from MasterDao contract
        // checking balance of a particular token of the `address`
        // basically, determines the "logic" of the strategy

        Ok(1)
    }
}
