use crate::traits::dao::DaoContractRef;
pub use crate::traits::proposal::*;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Storage, String, Balance},
};

use ink::prelude::vec::Vec;

use super::dao::FOUNDER;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
///Data of the Proposal SC
pub struct Data {
    pub master_dao: AccountId,
    pub proposals: Mapping<ProposalId, ProposalData>,
    pub votes: Mapping<(ProposalId, AccountId), VoteType>,
    pub proposal_id: ProposalId,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            master_dao: [0u8; 32].into(),
            proposals: Default::default(),
            votes: Default::default(),
            proposal_id: Default::default(),
        }
    }
}
///Default implementation of `modular_dao::traits::Proposal` SC
impl<T: Storage<Data>> Proposal for T {
    default fn propose(
        &mut self,
        title: String,
        description: String,
        duration: TimePeriod,
        quorum: u32,
        private_voting: bool,
        account_to: Option<AccountId>,
        amount: Option<Balance>,
        token_address: Option<AccountId>,
    ) -> Result<(), Error> {
        // TODO: logic if caller is allowed to propose (for only checks if caller has any power)
        // it could be part of voting strategy or seperate strategy
        if DaoContractRef::get_vote_weight(&self.data().master_dao, Self::env().caller()).unwrap_or_default().is_none()
        {
            return Err(Error::NoVotePower);
        }

        if duration.to_timestamp() < ONE_MINUTE {
            return Err(Error::ProposalTime);
        }

        let now = Self::env().block_timestamp();
        let proposal = ProposalData {
            title,
            description,
            vote_start: now,
            vote_end: now + duration.to_timestamp(),
            voters: Vec::new(),
            result: None,
            quorum,
            private_voting,
            status: Status::Active,
            account_to,
            amount,
            token_address,
        };

        let id = self.data().proposal_id;
        self.data().proposal_id += 1;
        self.data().proposals.insert(&id, &proposal);

        Ok(())
    }
    ///Returns proposal data
    default fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, Error> {
        self.data()
            .proposals
            .get(&id)
            .ok_or(Error::ProposalNotExists)
    }

    default fn count_votes(&mut self, id: ProposalId) -> Result<ProposalResult, Error> {
        let proposal = self
            .data()
            .proposals
            .get(&id)
            .ok_or(Error::ProposalNotExists)?;
        //check if there's already some result
        if proposal.result.is_some() {
            return Err(Error::VotesAlreadyCounted);
        }
        //check if proposal time passed
        let now = Self::env().block_timestamp();
        if now < proposal.vote_end {
            return Err(Error::ProposalTime);
        }

        let mut for_votes = 0;
        let mut against_votes = 0;
        let mut abstain_votes = 0;
        let mut vote_no = 0;
        if proposal.private_voting {
            todo!()

            // get result from private-voting module and
            // check quorum
            // update proposal.status
            // update proposal.result
        } else {
            for voter in &proposal.voters {
                if !DaoContractRef::has_delegated(&self.data().master_dao, *voter) {
                    let vote_weight =
                        DaoContractRef::get_vote_weight(&self.data().master_dao, *voter)
                            .unwrap_or_default();
                    if vote_weight.is_some() {
                        match self.data().votes.get(&(id, *voter)).unwrap_or_default() {
                            VoteType::For => {
                                for_votes = for_votes + vote_weight.unwrap_or_default()
                            }
                            VoteType::Against => {
                                against_votes = against_votes + vote_weight.unwrap_or_default()
                            }
                            _ => abstain_votes = abstain_votes + vote_weight.unwrap_or_default(),
                        }
                        vote_no += 1;
                    }
                }
            }
            //for delegator in DaoContractRef::get_delegators - calculate delegated votes
            //if
            let mut new_status = Status::Pending;
            let result: ProposalResult = ProposalResult(abstain_votes, for_votes, against_votes);
            //check quorum
            if proposal.quorum < vote_no as u32 {
                new_status = Status::Rejected;
            }

            //check if rejected
            if result.1 < result.2 {
                new_status = Status::Rejected;
            }

            self.data().proposals.insert(
                &id,
                &ProposalData {
                    result: Some(result.clone()),
                    status: new_status,
                    ..proposal
                },
            );
            Ok(result)
        }
    }
    ///executes the proposal
    default fn execute(&mut self, id: ProposalId) -> Result<(), Error> {
        let proposal = self
            .data()
            .proposals
            .get(&id)
            .ok_or(Error::ProposalNotExists)?;

        if proposal.status == Status::Rejected {
            self.data().proposals.insert(
                &id,
                &ProposalData {
                    status: Status::Archived,
                    ..proposal
                },
            );
            return Ok(());
        }

        if proposal.status != Status::Pending {
            return Err(Error::ProposalIsNotPending);
        }

        //TODO: emit appropriate event
        //ATM it's not possible to have shared event definition across smart contracts
        todo!()
    }
    ///Allows user to vote for on the specified proposal
    default fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), Error> {
        let vote_weight =
            DaoContractRef::get_vote_weight(&self.data().master_dao, Self::env().caller())
                .unwrap_or_default();
        //check if caller has right to vote
        if vote_weight.is_none() {
            return Err(Error::NoVotePower);
        }
        //check if has delegated vote
        if DaoContractRef::has_delegated(&self.data().master_dao, Self::env().caller()) {
            return Err(Error::DelegatedVote);
        }

        let mut proposal = self
            .data()
            .proposals
            .get(&id)
            .ok_or(Error::ProposalNotExists)?;

        if proposal.status != Status::Active {
            return Err(Error::ProposalIsNotActive);
        }

        let now = Self::env().block_timestamp();

        if now > proposal.vote_end {
            return Err(Error::ProposalTime);
        }
        if proposal.private_voting {
            return Err(Error::PrivateVoting);
        }
        self.data().votes.insert(&(id, Self::env().caller()), &vote);
        proposal.voters.push(Self::env().caller());
        self.data().proposals.insert(&id,&ProposalData {
            ..proposal
        });
        Ok(())
    }
    ///
    default fn vote_private(
        &mut self,
        id: ProposalId,
        _vote: VoteType,
        _secret: String,
    ) -> Result<(), Error> {
        let proposal = self
            .data()
            .proposals
            .get(&id)
            .ok_or(Error::ProposalNotExists)?;

        if proposal.status != Status::Active {
            return Err(Error::ProposalIsNotActive);
        }

        let now = Self::env().block_timestamp();

        if now < proposal.vote_end {
            return Err(Error::ProposalTime);
        }
        if !proposal.private_voting {
            return Err(Error::PrivateVoting);
        }
        todo!()
    }
    ///Returns `true` if `address` voted in any pending proposal
    default fn in_active_proposal(&self, account: AccountId) -> bool {
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
    ///Allows to "liberum veto" a proposal
    default fn liberum_veto(&mut self, id: ProposalId) -> Result<(), Error> {
        if DaoContractRef::liberum_veto_allowed(&self.data::<Data>().master_dao) {
            if DaoContractRef::has_role(
                &self.data::<Data>().master_dao,
                FOUNDER, //only MEMBERs can veto, could be changed to strategy or something
                Self::env().caller(),
            ) {
                let proposal = self
                    .data()
                    .proposals
                    .get(&id)
                    .ok_or(Error::ProposalNotExists)?;

                if proposal.status != Status::Active {
                    return Err(Error::ProposalIsNotActive);
                }

                let now = Self::env().block_timestamp();

                if now < proposal.vote_end {
                    return Err(Error::ProposalTime);
                }
                self.data().proposals.insert(
                    &id,
                    &ProposalData {
                        status: Status::Rejected,
                        ..proposal
                    },
                );
                Ok(())
            } else {
                return Err(Error::NotAllowedToVeto);
            }
        } else {
            return Err(Error::NotAllowedToVeto);
        }
    }
}
