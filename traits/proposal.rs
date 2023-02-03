use ink_env::AccountId;

use openbrush::traits::{Balance, String, Timestamp};

use ink_storage::traits::*;
use scale::{Decode, Encode};

use ink_prelude::vec::Vec;

pub const ONE_MINUTE: u64 = 60 * 1000;

#[openbrush::trait_definition]
pub trait Proposal {
    #[ink(message)]
    fn propose(
        &mut self,
        title: String,
        description: String,
        duration: u64,
    ) -> Result<(), ProposalError>;

    #[ink(message)]
    fn get_proposal(&self, id: ProposalId) -> Result<ProposalData, ProposalError>;

    #[ink(message)]
    fn execute(&mut self, id: ProposalId) -> Result<ProposalResult, ProposalError>;

    #[ink(message)]
    fn vote(&mut self, id: ProposalId, vote: VoteType) -> Result<(), ProposalError>;

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

#[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout, Copy)
)]
pub struct ProposalResult(pub u32, pub Balance, pub Balance);

impl Clone for ProposalResult {
    fn clone(&self) -> Self {
        ProposalResult(self.0, self.1, self.2)
    }
}

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

pub struct ProposalData {
    pub title: String,
    pub description: String,
    pub vote_start: Timestamp,
    pub vote_end: Timestamp,
    pub voters: Vec<AccountId>,
    pub result: Option<ProposalResult>,
}
