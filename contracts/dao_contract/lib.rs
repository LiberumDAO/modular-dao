#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod dao_contract {
    use openbrush::traits::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{traits::*};
    use modular_dao::traits::{dao_master::*,strategy::*};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DaoContract {
        name: String,
        strategies: Vec<AccountId>,
    }

    ///implementing DaoMaster trait
    impl DaoMaster for DaoContract {
        #[ink(message)]
        fn get_name(&self) -> String {
             self.name.clone()
        }

        #[ink(message)]
        fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(),String>  { //DaoError> {
            //logic to add module
            self.strategies.push(strategy_address);
            Ok(())
        }

        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Balance,String> { // //DaoError> {
            //logic to add module
            let mut total: Balance = 0;
            for strategy in &self.strategies {
                //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
                total = total + StrategyRef::get_vote_weight(strategy,address).unwrap_or_default();
            }
            Ok(total)
        }

    }

    impl DaoContract {
        
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.name = String::from(name);
            })
        }
    }
}
