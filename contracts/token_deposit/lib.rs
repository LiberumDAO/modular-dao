#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod token_deposit {
    use ink_storage::{traits::SpreadAllocate};
    use openbrush::storage::Mapping;
    use openbrush::traits::String;
    use openbrush::contracts::traits::psp22::*;
    use modular_dao::traits::deposit::*;
    

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TokenDeposit {
        master_dao: AccountId,
        token_address: AccountId,
        balances: Mapping<AccountId,Balance>
    }

    ///trait implementation
    impl Deposit for TokenDeposit {
        #[ink(message)]
        fn make_deposit(&self, amount: Balance) -> Result<(),String> {
            Ok(())
        }
    
        #[ink(message)]
        fn withdraw_deposit(&self, amount: Balance) -> Result<(),String> {
            Ok(())
        }
    
        #[ink(message)]
        fn get_deposit(&self, account: AccountId) -> Result<Balance,String> {
            self.balances.get(account)
        }
    }
    impl TokenDeposit {
        /// Constructor
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, token_address: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.master_dao = master_dao;
                instance.token_address = gov_token;
            })
        }
    }

}
