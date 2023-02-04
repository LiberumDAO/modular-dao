#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy1 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use modular_dao::impls::strategy;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default,SpreadAllocate, Storage)]
    pub struct Strategy {
        #[storage_field]
        data: strategy::Data,
        factor: u128,
        gov_token: AccountId,
    }

    ///trait implementation
    impl strategy::Strategy for Strategy {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Balance, strategy::Error> {
            //the logic could include getting some values from MasterDao contract
            //checking balance of a particular token of the `address`

            //just dummy calculation  with some balance of PSP22 token
            Ok(PSP22Ref::balance_of(&self.gov_token,address) * self.factor)
        }
    }
    impl Strategy {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128, gov_token: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.master_dao = master_dao;
                instance.factor = factor;
                instance.gov_token = gov_token;
            })
        }
    }

}
