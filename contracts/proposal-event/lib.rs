#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod proposal_event {
    use modular_dao::impls::proposal::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ProposalEvent {
        #[storage_field]
        data: Data,
    }

    impl Proposal for ProposalEvent {}

    impl ProposalEvent {
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.master_dao = master_dao;
            instance
        }
    }
}
