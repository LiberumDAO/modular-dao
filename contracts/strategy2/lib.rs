#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod strategy2 {
    use ink_storage::traits::SpreadAllocate;
    use modular_dao::impls::strategy::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Strategy2 {
        #[storage_field]
        data: Data,
        factor: u128,
    }

    ///trait implementation
    impl Strategy for Strategy2 {}
    impl Strategy2 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, factor: u128) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.master_dao = master_dao;
                instance.factor = factor;
            })
        }
    }
}
