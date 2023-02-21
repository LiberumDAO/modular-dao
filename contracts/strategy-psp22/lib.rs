#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy_psp22 {
    use openbrush::contracts::psp22::*;
    use modular_dao::impls::strategy;
    use modular_dao::impls::strategy::extensions::gov22;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive( Storage)]
    pub struct GOV22Contract {
        #[storage_field]
        strategy: strategy::Data,
        #[storage_field]
        gov22: gov22::Data,
        factor: u128,
    }

    impl Default for GOV22Contract {
        fn default() -> Self {
            Self {
                strategy: Default::default(),
                gov22: Default::default(),
                factor: 1,
            }
        }
    }

    impl gov22::GOV22 for GOV22Contract {}
    ///trait implementation
    impl strategy::Strategy for GOV22Contract {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Option<u128> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`
            let balance = PSP22Ref::balance_of(&self.gov22.token_address, address);
            //just dummy calculation  with some balance of PSP22 token
            if balance > 0 {
                return Some(balance * self.factor);
            }
            None
        }
    }
    impl GOV22Contract {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128, gov_token: AccountId) -> Self {
            let mut instance = Self::default();
                instance.strategy.master_dao = master_dao;
                instance.factor = factor;
                instance.gov22.token_address = gov_token;
                instance
        }
    }

}
