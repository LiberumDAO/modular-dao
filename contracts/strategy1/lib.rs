#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy1 {
    use ink_storage::{traits::SpreadAllocate};
    use openbrush::contracts::traits::psp22::*;
    use modular_dao::traits::{strategy::*};
    

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Strategy1 {
        master_dao: AccountId,
        factor: u128,
        gov_token: AccountId,
    }

    ///trait implementation
    impl Strategy for Strategy1 {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Balance, StrategyError> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`

            //just dummy calculation  with some balance of PSP22 token
            Ok(PSP22Ref::balance_of(&self.gov_token,address) * self.factor)
        }
    }
    impl Strategy1 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128, gov_token: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.master_dao = master_dao;
                instance.factor = factor;
                instance.gov_token = gov_token;
            })
        }
    }

}
