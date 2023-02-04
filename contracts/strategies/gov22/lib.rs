#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod gov22 {
    use openbrush::contracts::traits::psp22::PSP22Ref;
    use ink_storage::traits::SpreadAllocate;
    use modular_dao::impls::strategy;
    use openbrush::traits::Storage;

    /// 
    /// 
    #[ink(storage)]
    #[derive(Default,SpreadAllocate, Storage)]
    pub struct EqualStrategyContract {
        ///Strategy data: the `data.data` represents PSP22 token address
        #[storage_field]
        data: strategy::Data<AccountId>,
    }

    ///Strategy implementation
    impl strategy::Strategy for EqualStrategyContract {
        /// Returns `balance_of` address for strategy PSP22 token
        #[ink(message)]
        fn get_vote_weight(&self, _address: AccountId) -> Result<Balance, strategy::Error> {
            Ok(PSP22Ref::balance_of(&self.data.data,self.data.data))
        }
    }

    impl EqualStrategyContract {
        /// Constructor
        /// token_address should reference to a smart contract that is a PSP22 standard
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, token_address: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.master_dao = master_dao;
                instance.data.data = token_address;
            })
        }
    }

}
