#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod strategy1 {
    use ink_storage::{traits::SpreadAllocate};
    use openbrush::traits::String;

    use modular_dao::traits::{strategy::*};
    

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Strategy1 {
        master_dao: AccountId,
        factor: u128,
    }

    ///trait implementation
    impl Strategy for Strategy1 {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Balance, String> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`

            //just dummy calculation 
            Ok(10 * self.factor)
        }
    }
    impl Strategy1 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.master_dao = master_dao;
            })
        }
    }

}
