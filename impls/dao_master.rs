use openbrush::traits::{AccountId, String, Balance, Storage};
use ink_prelude::vec::Vec;
use crate::traits::proposal::ProposalRef;
use crate::traits::strategy::StrategyRef;
pub use crate::traits::dao_master::*;


#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default,Debug)]
///SC Data
pub struct Data {
    pub name: String,
    pub strategies: Vec<AccountId>,
    pub proposal_types: Vec<AccountId>,
}
///Default implementation of the `modular_dao::traits::DaoMaster`
impl<T: Storage<Data>> DaoMaster for T {

        default fn get_name(&self) -> String {
             self.data().name.clone()
        }

        /// TODO: `only owner`
        default fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(),DaoError>  { 
            self.data().strategies.push(strategy_address);
            Ok(())
        }
        ///TODO: `only owner`
        default fn add_proposal_type(&mut self, proposal_address: AccountId) -> Result<(),DaoError>  { 
            self.data().proposal_types.push(proposal_address);
            Ok(())
        }

        default fn get_vote_weight(&self, address: AccountId) -> Result<Balance,DaoError> { 
            //logic to add module
            let mut total: Balance = 0;
            for strategy in &self.data().strategies {
                //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
                total = total + StrategyRef::get_vote_weight(strategy,address).unwrap_or_default();
            }
            Ok(total)
        }

        default fn in_active_proposal(&self, address: AccountId) -> bool {
            for proposal_type in &self.data().proposal_types {
                //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
                if ProposalRef::in_active_proposal(proposal_type, address) { return true };
            }
            false
        }
}
