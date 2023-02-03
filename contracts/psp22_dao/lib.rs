#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp22_dao {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{contracts::psp22::*, traits::Storage};

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Psp22Dao {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        wrapper: openbrush::contracts::psp22::extensions::wrapper::Data,
        // fields for custom logic here
    }

    impl PSP22 for Psp22Dao {
        // customizations here
    }

    impl openbrush::contracts::psp22::extensions::wrapper::PSP22Wrapper for Psp22Dao {
        //custom logic here
    }

    impl Psp22Dao {
        ///it will depend on necessary data and further logic implementation
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Psp22Dao| {
                instance
                    ._mint_to(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }
    }
}
