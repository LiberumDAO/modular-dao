#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy_whitelist {
    use modular_dao::impls::strategy;
    use modular_dao::impls::strategy::extensions::whitelist;
    use openbrush::traits::Storage;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    #[derive( Storage)]
    pub struct WhitelistContract {
        #[storage_field]
        strategy: strategy::Data,
        #[storage_field]
        whitelist: whitelist::Data,
        factor: u128,
    }

    impl Default for WhitelistContract {
        fn default() -> Self {
            Self {
                strategy: Default::default(),
                whitelist: Default::default(),
                factor: Default::default(),
            }
        }
    }

    impl whitelist::Whitelist for WhitelistContract {}


    ///trait implementation
    impl strategy::Strategy for WhitelistContract {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Option<u128> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`

            //just dummy calculation  with some balance of PSP22 token
            if self.whitelist.list.contains(&address) {
                return Some(self.factor)
            }
            None
        }
    }
    impl WhitelistContract {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128, list: Vec<AccountId>) -> Self {
            let mut instance = Self::default();
                instance.strategy.master_dao = master_dao;
                instance.factor = factor;
                instance.whitelist.list = list;
                instance
        }
    }

}
