#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod proposal1 {
    use ink_storage::{traits::*};
    use modular_dao::impls::proposal::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default,SpreadAllocate, Storage)]
    pub struct Proposal1 {
        #[storage_field]
        data: Data,
    }

    impl Proposal for Proposal1 {}

    impl Proposal1 {
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.master_dao = master_dao;
            })
        }
    }
}
