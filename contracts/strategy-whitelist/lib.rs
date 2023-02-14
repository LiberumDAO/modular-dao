#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy_whitelist {
    use openbrush::contracts::traits::access_control::*;
    use modular_dao::impls::strategy;
    use modular_dao::impls::dao::MEMBER;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive( Storage)]
    pub struct Whitelist {
        #[storage_field]
        data: strategy::Data,
        factor: u128,
        gov_token: AccountId,
    }

    impl Default for Whitelist {
        fn default() -> Self {
            Self {
                data: Default::default(),
                factor: Default::default(),
                gov_token: [0u8; 32].into(),
            }
        }
    }


    ///trait implementation
    impl strategy::Strategy for Whitelist {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Option<u128>, strategy::Error> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`

            //just dummy calculation  with some balance of PSP22 token
            if AccessControlRef::has_role(&self.data.master_dao, MEMBER,address) {
                return Ok(Some(1))
            }
            Ok(None)
        }
    }
    impl Whitelist {
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
