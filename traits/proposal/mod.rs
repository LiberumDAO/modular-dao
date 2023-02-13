use openbrush::traits::AccountId;

use openbrush::traits::{Balance, String, Timestamp};

//use ink_storage::traits::*;
use scale::{Decode, Encode};

use ink::prelude::vec::Vec;
#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

///Proposal SC
#[openbrush::trait_definition]
pub trait Proposal {
    #[ink(message)]
    fn propose(
        &mut self,
        title: String,
        description: String,
        duration: u64,
        quorum: u32,
        account_to: AccountId,
        private_voting: bool,
    ) -> Result<(), Error>;
    ///Returns proposal data
    #[ink(message)]
    fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, Error>;
    ///Updates result for the proposal if it hasn't been done yet
    #[ink(message)]
    fn count_votes(&mut self, id: ProposalId) -> Result<ProposalResult, Error>;
    ///executes the proposal
    #[ink(message)]
    fn execute(&mut self, id: ProposalId) -> Result<(), Error>;
    ///Allows user to vote for on the specified proposal
    #[ink(message)]
    fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), Error>;
    ///Returns `true` if `address` voted in any pending proposal
    #[ink(message)]
    fn in_active_proposal(&self, account: AccountId) -> bool;
}

#[openbrush::wrapper]
pub type ProposalRef = dyn Proposal;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    SomeError,
}

pub type ProposalId = u32;

#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
///Proposal result in a form (number of voters, For votes, Against votes)
pub struct ProposalResult(pub u32, pub Balance, pub Balance);


#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(Eq, PartialEq,StorageLayout, scale_info::TypeInfo))]
pub enum VoteType {
    For,
    Against,
    Abstain,
}


impl Default for VoteType {
    fn default() -> Self {
        VoteType::Abstain
    }
}

#[derive(Encode, Decode)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, scale_info::TypeInfo)
)]
pub struct Vote(pub AccountId, pub VoteType);




#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
///Struct representing a single proposal's data
pub struct ProposalData {
    pub title: String,
    pub description: String,
    pub vote_start: Timestamp,
    pub vote_end: Timestamp,
    pub voters: Vec<AccountId>,
    pub result: Option<ProposalResult>,
    pub quorum: u32,
    pub account_to: AccountId,
    pub private_voting: bool,
    pub executed: bool,
}
