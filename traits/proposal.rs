use ink_env::AccountId;

use openbrush::traits::{Balance, String, Timestamp};

use ink_storage::traits::*;
use scale::{Decode, Encode};

use ink_prelude::vec::Vec;

pub const ONE_MINUTE: u64 = 60 * 1000;
///Proposal SC
#[openbrush::trait_definition]
pub trait Proposal {
    #[ink(message)]
    fn propose(
        &mut self,
        title: String,
        description: String,
        duration: u64,
    ) -> Result<(), ProposalError>;
    ///Returns proposal data
    #[ink(message)]
    fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, ProposalError>;
    ///executes the proposal
    #[ink(message)]
    fn execute(&mut self, id: ProposalId) -> Result<ProposalResult, ProposalError>;
    ///Allows user to vote for on the specified proposal
    #[ink(message)]
    fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), ProposalError>;
    ///Returns `true` if `address` voted in any pending proposal
    #[ink(message)]
    fn in_active_proposal(&self, account: AccountId) -> bool;
}

#[openbrush::wrapper]
pub type ProposalRef = dyn Proposal;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ProposalError {
    SomeError,
}

pub type ProposalId = u32;

#[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default, Clone, Copy)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
)]
///Proposal result in a form (number of voters, For votes, Against votes)
pub struct ProposalResult(pub u32, pub Balance, pub Balance);


#[derive(Encode, Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

impl SpreadAllocate for VoteType {
    fn allocate_spread(_: &mut KeyPtr) -> Self {
        VoteType::Abstain
    }
}

impl Default for VoteType {
    fn default() -> Self {
        VoteType::Abstain
    }
}

#[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
)]
pub struct Vote(pub AccountId, pub VoteType);

#[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default, Clone)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, StorageLayout, scale_info::TypeInfo)
)]
///Struct representing a single proposal's data
pub struct ProposalData {
    pub title: String,
    pub description: String,
    pub vote_start: Timestamp,
    pub vote_end: Timestamp,
    pub voters: Vec<AccountId>,
    pub result: Option<ProposalResult>,
}
