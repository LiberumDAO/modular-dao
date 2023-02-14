use crate::traits::{dao, proposal::ProposalRef, strategy::StrategyRef};
use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, Balance, OccupiedStorage, Storage};
use openbrush::{contracts::access_control::*, modifiers};

use ink::storage::traits::{ManualKey, ResolverKey, Storable, StorableHint};

pub const FOUNDER: RoleType = ink::selector_id!("FOUNDER");
pub const MEMBER: RoleType = ink::selector_id!("MEMBER");

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
///SC Data
pub struct Data {
    ///stores `AccountId` of integrated strategies
    pub strategies: Vec<AccountId>,
    ///stores `AccountId` of integrated proposal types
    pub proposal_types: Vec<AccountId>,
    ///
    pub private_voting: bool,
    ///
    pub liberum_veto: bool,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            strategies: Default::default(),
            proposal_types: Default::default(),
            private_voting: false,
            liberum_veto: false,
        }
    }
}

///Default implementation of the `modular_dao::traits::Dao`
impl<T, M> dao::Dao for T
where
    T: Storage<Data> + Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
    M: members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + StorableHint<
            ResolverKey<
                <M as StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>>::PreferredKey,
                ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>,
            >,
            Type = M,
        >,
{

    default fn private_voting_allowed(&self) -> bool{
        self.data::<Data>().private_voting
    }

    default fn liberum_veto_allowed(&self) -> bool{
        self.data::<Data>().liberum_veto
    }

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
    default fn get_vote_weight(&self, address: AccountId) -> Result<Option<u128>, dao::Error> {
        //logic to add module
        let mut total: Balance = 0;
        for strategy in &self.data::<Data>().strategies {
            //read and sum the vote weight for each strategy by calling other contract that implement Strategy trait
            let strategy_weight = StrategyRef::get_vote_weight(strategy, address).unwrap_or_default();
            if strategy_weight.is_none() {
                return Ok(None)
            }
            total = total + strategy_weight.unwrap_or_default();
        }
        Ok(Some(total))
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

    #[modifiers(only_role(FOUNDER))]
    default fn grant_role_in_dao(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        if self.data::<access_control::Data<M>>().members.has_role(role, &account) {
            return Err(AccessControlError::RoleRedundant)
        }
        self.data::<access_control::Data<M>>().members.add(role, &account);
        Ok(())
    }
    
    #[modifiers(only_role(FOUNDER))]
    default fn revoke_role_in_dao(&mut self, role: RoleType, account: AccountId) -> Result<(), AccessControlError> {
        check_role(self, role, account)?;
        self.data::<access_control::Data<M>>().members.remove(role, &account);
        Ok(())
    }

    
}
