#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod token_deposit {
    use ink_storage::traits::SpreadAllocate;
    use modular_dao::impls::deposit::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct TokenDeposit {
        #[storage_field]
        data: Data,
    }

    ///trait implementation
    impl Deposit for TokenDeposit {}
    impl TokenDeposit {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, token_address: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.dao_master = master_dao;
                instance.data.token_address = token_address;
            })
        }
    }
}
