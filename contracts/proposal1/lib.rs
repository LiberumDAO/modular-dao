#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod proposal1 {
    use ink_prelude::vec::Vec;
    use ink_storage::{traits::*, Mapping};
    use modular_dao::traits::{dao_master::DaoMasterRef, proposal::*};
    use openbrush::traits::DefaultEnv;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Proposal1 {
        master_dao: AccountId,
        proposals: Mapping<ProposalId, ProposalData>,
        votes: Mapping<(ProposalId, AccountId), VoteType>,
        proposal_id: ProposalId,
    }

    ///implementing DaoMaster trait
    impl Proposal for Proposal1 {
        #[ink(message)]
        fn propose(
            &mut self,
            title: String,
            description: String,
            duration: u64,
        ) -> Result<(), ProposalError> {
            //TODO: logic if caller is allowed to propose (for only checks if caller has any power)
            //it could be part of voting strategy or seperate strategy
            if DaoMasterRef::get_vote_weight(&self.master_dao, Self::env().caller())
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

            let id = self.proposal_id;
            self.proposal_id += 1;
            self.proposals.insert(id, &proposal);

            Ok(())
        }

        #[ink(message)]
        fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, ProposalError> {
            self.proposals.get(id).ok_or(ProposalError::SomeError)
        }

        #[ink(message)]
        fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), ProposalError> {
            self.votes.insert((id, Self::env().caller()), &vote);
            Ok(())
        }

        #[ink(message)]
        fn execute(&mut self, id: ProposalId) -> Result<ProposalResult, ProposalError> {
            let proposal = self.proposals.get(&id).ok_or(ProposalError::SomeError)?;
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
                let vote_weight = DaoMasterRef::get_vote_weight(&self.master_dao, *voter)
                .unwrap_or_default();
                match self.votes.get((id, voter)).unwrap_or_default() {
                    VoteType::For => {for_votes = for_votes + vote_weight},
                    VoteType::Against => {against_votes = against_votes + vote_weight},
                    _ => ()
                }
            }

            let result: ProposalResult = ProposalResult(
                proposal.voters.len().try_into().unwrap_or_default(),
                for_votes,
                against_votes,
            );
            self.proposals.get(&id).ok_or(ProposalError::SomeError)?;
            self.proposals.insert(
                id,
                &ProposalData {
                    result: Some(result.clone()),
                    ..proposal
                },
            );

            //TODO: emit appropriate event

            Ok(result)
        }
    }

    impl Proposal1 {
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.master_dao = master_dao;
            })
        }
    }
}
