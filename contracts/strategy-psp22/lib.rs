#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy_psp22 {
    use openbrush::contracts::psp22::*;
    use modular_dao::impls::strategy;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive( Storage)]
    pub struct GOV22 {
        #[storage_field]
        data: strategy::Data,
        factor: u128,
        gov_token: AccountId,
    }

    impl Default for GOV22 {
        fn default() -> Self {
            Self {
                data: Default::default(),
                factor: Default::default(),
                gov_token: [0u8; 32].into(),
            }
        }
    }


    ///trait implementation
    impl strategy::Strategy for GOV22 {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Option<u128>, strategy::Error> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`
            let balance = PSP22Ref::balance_of(&self.gov_token, address);
            //just dummy calculation  with some balance of PSP22 token
            if balance > 0 {
                return Ok(Some(balance * self.factor));
            }
            Ok(None)
        }
    }
    impl GOV22 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128, gov_token: AccountId) -> Self {
            let mut instance = Self::default();
                instance.data.master_dao = master_dao;
                instance.factor = factor;
                instance.gov_token = gov_token;
                instance
        }
    }

}
