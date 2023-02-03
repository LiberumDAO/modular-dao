#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod dao_contract {
    use openbrush::traits::String;
    use ink_storage::traits::SpreadAllocate;
    use modular_dao::impls::dao_master::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default,SpreadAllocate, Storage)]
    pub struct DaoContract {
        #[storage_field]
        data: Data,
    }

    ///implementing DaoMaster trait
    impl DaoMaster for DaoContract { }

    impl DaoContract {
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.name = String::from(name);
            })
        }

    }
}
