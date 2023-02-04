#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod dao {
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use modular_dao::impls::dao::{self, FOUNDER};
    use modular_dao::traits::dao::*;
    use openbrush::{contracts::access_control::*, traits::Storage};

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct DaoContract {
        #[storage_field]
        dao: dao::Data,
        #[storage_field]
        access: access_control::Data,
    }

    impl Dao for DaoContract {}

    impl DaoContract {
        #[ink(constructor)]
        pub fn new(founders: Vec<AccountId>) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_admin(caller); // <- not sure what it does, couldn't find docs about it
                if founders.contains(&caller) {
                    for i in 0..founders.len() {
                        instance
                        .grant_role(FOUNDER, *founders.get(i).unwrap())
                        .expect("Should grant the role");
                    }
                } else {
                    instance
                        .grant_role(FOUNDER, caller)
                        .expect("Should grant the role");
                }
            })
        }
    }
}
