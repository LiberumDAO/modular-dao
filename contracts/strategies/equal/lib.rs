#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod equal {
    use ink_storage::traits::SpreadAllocate;
    use modular_dao::impls::strategy;
    use openbrush::traits::Storage;

    type VoteValue = u128;
    /// Simplest strategy contract
    /// Anyone can vote - vote value is equal for vote
    #[ink(storage)]
    #[derive(Default,SpreadAllocate, Storage)]
    pub struct EqualStrategyContract {
        ///Strategy data: the `data.data` represents Vote Value
        #[storage_field]
        data: strategy::Data<VoteValue>,
    }

    ///Strategy implementation
    impl strategy::Strategy for EqualStrategyContract {
        /// Returns constant vote value for any address 
        #[ink(message)]
        fn get_vote_weight(&self, _address: AccountId) -> Result<Balance, strategy::Error> {
            Ok(self.data.data)
        }
    }

    impl EqualStrategyContract {
        /// Constructor
        /// vote_value determines vote value for every vote
        #[ink(constructor)]
        pub fn new(master_dao: AccountId, vote_value: VoteValue) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.data.master_dao = master_dao;
                instance.data.data = vote_value;
            })
        }
    }

}
