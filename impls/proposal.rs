use crate::traits::dao_master::DaoMasterRef;
pub use crate::traits::proposal::*;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Storage, String},
};

use ink_prelude::vec::Vec;

#[openbrush::upgradeable_storage(openbrush::storage_unique_key!(Data))]
#[derive(Default, Debug)]
///Data of the Proposal SC
pub struct Data {
    pub master_dao: AccountId,
    pub proposals: Mapping<ProposalId, ProposalData>,
    pub votes: Mapping<(ProposalId, AccountId), VoteType>,
    pub proposal_id: ProposalId,
}
///Default implementation of `modular_dao::traits::Proposal` SC
impl<T: Storage<Data>> Proposal for T {
    default fn propose(
        &mut self,
        title: String,
        description: String,
        duration: u64,
    ) -> Result<(), ProposalError> {
        // TODO: logic if caller is allowed to propose (for only checks if caller has any power)
        // it could be part of voting strategy or seperate strategy
        if DaoMasterRef::get_vote_weight(&self.data().master_dao, Self::env().caller())
            .unwrap_or_default()
            == 0
        {
            return Err(ProposalError::SomeError);
        }

        if duration == 0 || duration > 60 * ONE_MINUTE {
            return Err(ProposalError::SomeError);
        }

        let now = Self::env().block_timestamp();
        let proposal = ProposalData {
            title,
            description,
            vote_start: now,
            vote_end: now + duration * ONE_MINUTE,
            voters: Vec::new(),
            result: None,
        };

        let id = self.data().proposal_id;
        self.data().proposal_id += 1;
        self.data().proposals.insert(&id, &proposal);

        Ok(())
    }
    ///Returns proposal data
    fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, ProposalError> {
        self.data()
            .proposals
            .get(&id)
            .ok_or(ProposalError::SomeError)
    }
    ///executes the proposal
    fn execute(&mut self, id: ProposalId) -> Result<ProposalResult, ProposalError> {
        let proposal = self
            .data()
            .proposals
            .get(&id)
            .ok_or(ProposalError::SomeError)?;
        if proposal.result.is_some() {
            return Err(ProposalError::SomeError);
        }

        let now = Self::env().block_timestamp();

        if now < proposal.vote_end {
            return Err(ProposalError::SomeError);
        }

        let mut for_votes = 0;
        let mut against_votes = 0;
        for voter in &proposal.voters {
            let vote_weight =
                DaoMasterRef::get_vote_weight(&self.data().master_dao, *voter).unwrap_or_default();
            match self.data().votes.get(&(id, *voter)).unwrap_or_default() {
                VoteType::For => for_votes = for_votes + vote_weight,
                VoteType::Against => against_votes = against_votes + vote_weight,
                _ => (),
            }
        }

        let result: ProposalResult = ProposalResult(
            proposal.voters.len().try_into().unwrap_or_default(),
            for_votes,
            against_votes,
        );
        self.data()
            .proposals
            .get(&id)
            .ok_or(ProposalError::SomeError)?;
        self.data().proposals.insert(
            &id,
            &ProposalData {
                result: Some(result.clone()),
                ..proposal
            },
        );

        //TODO: emit appropriate event

        Ok(result)
    }
    ///Allows user to vote for on the specified proposal
    fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), ProposalError> {
        self.data().votes.insert(&(id, Self::env().caller()), &vote);
        Ok(())
    }
    ///Returns `true` if `address` voted in any pending proposal
    fn in_active_proposal(&self, account: AccountId) -> bool {
        let now = Self::env().block_timestamp();
        for i in 0..self.data().proposal_id {
            if now < self.data().proposals.get(&i).unwrap().vote_end
                && self
                    .data()
                    .proposals
                    .get(&i)
                    .unwrap()
                    .voters
                    .contains(&account)
            {
                return true;
            };
        }
        false
    }
}
