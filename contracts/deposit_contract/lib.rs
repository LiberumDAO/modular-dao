#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod deposit_contract {
    use ink_storage::traits::SpreadAllocate;
    use modular_dao::impls::deposit::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default,SpreadAllocate, Storage)]
    pub struct DepositContract {
        #[storage_field]
        data: Data,
    }

    ///implementing DaoMaster trait
    impl Deposit for DepositContract { }

    impl DepositContract {
        #[ink(constructor)]
        pub fn new(dao_master: AccountId, token_address: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.dao_master = dao_master;
                instance.data.token_address = token_address;
            })
        }


    }
}
