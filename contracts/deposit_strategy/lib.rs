#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod deposit_strategy {
    use ink_storage::{traits::SpreadAllocate};
    use modular_dao::traits::{strategy::*};
    use modular_dao::traits::deposit::DepositRef;
    

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DepositStrategy {
        master_dao: AccountId,
        deposit: AccountId,
        factor: u128,
        gov_token: AccountId,
    }

    ///trait implementation
    impl Strategy for DepositStrategy {
        #[ink(message)]
        fn get_vote_weight(&self, address: AccountId) -> Result<Balance, StrategyError> {
            // the logic could include getting some values from MasterDao contract
            // checking balance of a particular token of the `address`
            // basically, determines the "logic" of the strategy

            // just dummy calculation  with some balance of PSP22 token
            Ok(DepositRef::deposit_of(&self.deposit,address).unwrap_or_default() * self.factor)
        }
    }
    impl DepositStrategy {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128, gov_token: AccountId, deposit: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.master_dao = master_dao;
                instance.deposit = deposit;
                instance.factor = factor;
                instance.gov_token = gov_token;
            })
        }
    }

}
