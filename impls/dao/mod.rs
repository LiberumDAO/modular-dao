pub mod roles_dao;
use crate::traits::{proposal::ProposalRef, strategy::StrategyRef, dao};
use ink_prelude::vec::Vec;
use openbrush::traits::{AccountId, Balance, Storage};
use openbrush::{contracts::access_control::*, modifiers, traits::OccupiedStorage};
pub const FOUNDER: RoleType = ink_lang::selector_id!("FOUNDER");

#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default, Debug)]
///SC Data
pub struct Data {
    ///stores `AccountId` of integrated strategies
    pub strategies: Vec<AccountId>,
    ///stores `AccountId` of integrated proposal types
    pub proposal_types: Vec<AccountId>,
}

///Default implementation of the `modular_dao::traits::Dao`
impl<T, M> dao::Dao for T
where
    T: Storage<Data> + Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
    M: members::MembersManager,
{
    ///allows Founders to add strategy to the DAO
    #[modifiers(only_role(FOUNDER))]
    default fn add_strategy(&mut self, strategy_address: AccountId) -> Result<(), dao::Error> {
        if self.data::<Data>().strategies.contains(&strategy_address) {
            return Err(dao::Error::StrategyAlreadyIncorporated);
        }
        self.data::<Data>().strategies.push(strategy_address);
        Ok(())
    }
    ///allows Founders to add proposal type to the DAO
    #[modifiers(only_role(FOUNDER))]
    default fn add_proposal_type(&mut self, proposal_address: AccountId) -> Result<(), dao::Error> {
        if self
            .data::<Data>()
            .proposal_types
            .contains(&proposal_address)
        {
            return Err(dao::Error::ProposalTypeAlreadyIncorporated);
        }
        self.data::<Data>().proposal_types.push(proposal_address);
        Ok(())
    }
    ///Calculates vote weight based on incorporated strategies at given moment
    ///Returns total vote weight for a given `AccountId`
    default fn get_vote_weight(&self, address: AccountId) -> Result<u128, dao::Error> {
        //logic to add module
        let mut total: Balance = 0;
        for strategy in &self.data::<Data>().strategies {
            //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
            total = total + StrategyRef::get_vote_weight(strategy, address).unwrap_or_default();
        }
        Ok(total)
    }
    ///Returns `true` if given `AccountId` has voted on at least one
    /// unresolved proposal
    default fn in_active_proposal(&self, address: AccountId) -> bool {
        for proposal_type in &self.data::<Data>().proposal_types {
            //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
            if ProposalRef::in_active_proposal(proposal_type, address) {
                return true;
            };
        }
        false
    }
}
