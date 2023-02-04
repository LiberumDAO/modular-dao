use crate::traits::dao::DaoRef;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Storage, String},
};
pub use crate::traits::proposal::*;

use ink_prelude::vec::Vec;

pub const ONE_MINUTE: u64 = 60 * 1000;

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
    ) -> Result<(), Error> {
        // TODO: logic if caller is allowed to propose (for only checks if caller has any power)
        // it could be part of voting strategy or seperate strategy
        if DaoRef::get_vote_weight(&self.data().master_dao, Self::env().caller())
            .unwrap_or_default()
            == 0
        {
            return Err(Error::SomeError);
        }

        if duration == 0 || duration > 60 * ONE_MINUTE {
            return Err(Error::SomeError);
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
    fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, Error> {
        self.data()
            .proposals
            .get(&id)
            .ok_or(Error::SomeError)
    }
    ///executes the proposal
    fn execute(&mut self, id: ProposalId) -> Result<ProposalResult, Error> {
        let proposal = self
            .data()
            .proposals
            .get(&id)
            .ok_or(Error::SomeError)?;
        if proposal.result.is_some() {
            return Err(Error::SomeError);
        }

        let now = Self::env().block_timestamp();

        if now < proposal.vote_end {
            return Err(Error::SomeError);
        }

        let mut for_votes = 0;
        let mut against_votes = 0;
        for voter in &proposal.voters {
            let vote_weight =
                DaoRef::get_vote_weight(&self.data().master_dao, *voter).unwrap_or_default();
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
            .ok_or(Error::SomeError)?;
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
    fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), Error> {
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
