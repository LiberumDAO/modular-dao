use openbrush::traits::{AccountId, String, Balance};
use ink_prelude::vec::Vec;
use crate::traits::strategy::StrategyRef;
pub use crate::traits::dao_master::*;


#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default,Debug)]
pub struct Data {
    pub name: String,
    pub strategies: Vec<AccountId>,
}

impl<T: openbrush::traits::Storage<Data>> DaoMaster for T {

        default fn get_name(&self) -> String {
             self.data().name.clone()
        }

        default fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(),String>  { //DaoError> {
            //logic to add module
            self.data().strategies.push(strategy_address);
            Ok(())
        }

        default fn get_vote_weight(&self, address: AccountId) -> Result<Balance,String> { // //DaoError> {
            //logic to add module
            let mut total: Balance = 0;
            for strategy in &self.data().strategies {
                //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
                total = total + StrategyRef::get_vote_weight(strategy,address).unwrap_or_default();
            }
            Ok(total)
        }
}
