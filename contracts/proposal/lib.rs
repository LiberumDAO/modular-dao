#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod proposal {
    use ink_storage::traits::*;
    use modular_dao::impls::proposal::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ProposalContract {
        #[storage_field]
        data: Data,
    }

    impl Proposal for ProposalContract {}

    impl ProposalContract {
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.master_dao = master_dao;
            })
        }
    }
}
