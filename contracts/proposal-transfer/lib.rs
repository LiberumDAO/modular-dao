#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod proposal_transfer {
    use ink::codegen::EmitEvent;
    use modular_dao::impls::proposal::*;
    use openbrush::traits::DefaultEnv;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ProposalContract {
        #[storage_field]
        data: Data,
    }

    impl Proposal for ProposalContract {}

    impl ProposalContract {
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.master_dao = master_dao;
            instance
        }
    }
    // due to how ink! currently handles events, it is impossible to implement events in default implementation
    impl Internal for ProposalContract {
        fn _emit_proposal_created_event(&self, proposal_id: ProposalId, creator: AccountId) {
            Self::env().emit_event(ProposalCreated {
                proposal_id,
                creator,
            })
        }
        fn _emit_voted_event(&self, proposal_id: ProposalId, voter: AccountId) {
            Self::env().emit_event(Voted { proposal_id, voter })
        }
        fn _emit_votes_counted_event(&self, proposal_id: ProposalId) {
            Self::env().emit_event(VotesCounted { proposal_id })
        }
        fn _emit_executed_event(&self, proposal_id: ProposalId, executor: AccountId) {
            Self::env().emit_event(Executed {
                proposal_id,
                executor,
            })
        }
        fn _emit_transferred_event(
            &self,
            proposal_id: ProposalId,
            account_to: AccountId,
            amount: Balance,
        ) {
            Self::env().emit_event(Transferred {
                proposal_id,
                account_to,
                amount,
            })
        }
    }
    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: ProposalId,

        #[ink(topic)]
        creator: AccountId,
    }

    #[ink(event)]
    pub struct Executed {
        #[ink(topic)]
        proposal_id: ProposalId,

        #[ink(topic)]
        executor: AccountId,
    }

    #[ink(event)]
    pub struct Transferred {
        #[ink(topic)]
        proposal_id: ProposalId,

        #[ink(topic)]
        account_to: AccountId,

        amount: Balance,
    }

    #[ink(event)]
    pub struct Voted {
        #[ink(topic)]
        proposal_id: ProposalId,

        #[ink(topic)]
        voter: AccountId,
    }

    #[ink(event)]
    pub struct VotesCounted {
        #[ink(topic)]
        proposal_id: ProposalId,
    }
}
