#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy2 {
    use ink_storage::{traits::SpreadAllocate};
    use modular_dao::traits::{strategy::*};
    

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Strategy2 {
        master_dao: AccountId,
        factor: u128,
    }

    ///trait implementation
    impl Strategy for Strategy2 {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Balance, StrategyError> { // StrategyError> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`

            //just dummy calculation 
            Ok(5 * self.factor)
        }
    }
    impl Strategy2 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.master_dao = master_dao;
                instance.factor = factor;
            })
        }
    }

}
