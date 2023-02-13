#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod dao_base {

    use modular_dao::impls::dao::{self, FOUNDER};
    use modular_dao::traits::dao::*;
    use openbrush::{contracts::access_control::*, traits::Storage};
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DaoContract {
        #[storage_field]
        dao: dao::Data,
        #[storage_field]
        access: access_control::Data,
    }

    impl AccessControl for DaoContract {}
    impl Dao for DaoContract {}

    impl DaoContract {
        #[ink(constructor)]
        pub fn new(founders: Vec<AccountId>) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            instance
                .grant_role(FOUNDER, caller)
                .expect("Should grant the role");
            for i in 0..founders.len() {
                if *founders.get(i).unwrap() != caller {
                    instance
                        .grant_role(FOUNDER, *founders.get(i).unwrap())
                        .expect("Should grant the role");
                }
            }
            instance
        }
    }
}
