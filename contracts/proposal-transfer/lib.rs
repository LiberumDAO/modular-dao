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
    pub struct ProposalEvent {
        #[storage_field]
        data: Data,
    }

    impl Proposal for ProposalEvent {
        #[ink(message)]
        fn execute(&mut self, id: ProposalId) -> Result<(), Error> {
            let proposal = self.data.proposals.get(&id).ok_or(Error::SomeError)?;

            if proposal.status == Status::Rejected {
                self.data.proposals.insert(
                    &id,
                    &ProposalData {
                        status: Status::Archived,
                        ..proposal
                    },
                );
                return Ok(());
            }

            if proposal.status != Status::Pending {
                return Err(Error::SomeError);
            }

            if Self::env().balance() < proposal.amount {
                return Err(Error::SomeError)
            }

            Self::env()
                .transfer(proposal.account_to, proposal.amount)
                .map_err(|_| Error::SomeError)?;
            
            self.data.proposals.insert(
                &id,
                &ProposalData {
                    status: Status::Executed,
                    ..proposal
                },
            );
            Self::env().emit_event(Executed {
                proposal_id: Some(id),
                executor: Some(Self::env().caller()),
                account_to: Some(proposal.account_to),
                amount: Some(proposal.amount),
            });
            Ok(())
        }
    }

    impl ProposalEvent {
        #[ink(constructor)]
        pub fn new(master_dao: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.master_dao = master_dao;
            instance
        }
    }

    #[ink(event)]
    pub struct Executed {
        #[ink(topic)]
        proposal_id: Option<ProposalId>,

        #[ink(topic)]
        executor: Option<AccountId>,

        #[ink(topic)]
        account_to: Option<AccountId>,

        amount: Option<Balance>,

    }
}
