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
        duration: TimePeriod,
        quorum: u32,
        private_voting: bool,
        account_to: Option<AccountId>,
        amount: Option<Balance>,
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
    ///Allows user to vote for on the specified proposal
    #[ink(message)]
    fn vote_private(&mut self, id: ProposalId, vote: VoteType, secret: String) -> Result<(), Error>;
    ///Returns `true` if `address` voted in any pending proposal
    #[ink(message)]
    fn in_active_proposal(&self, account: AccountId) -> bool;
    #[ink(message)]
    fn liberum_veto(&mut self, id: ProposalId) -> Result<(),Error>;
}

#[openbrush::wrapper]
pub type ProposalRef = dyn Proposal;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AlreadyVoted,
    NoVotePower,
    ProposalTime,
    ProposalNotExists,
    VotesAlreadyCounted,
    ProposalIsNotPending,
    ProposalIsNotActive,
    PrivateVoting,
    NotAllowedToVeto,
    DelegatedVote,
    NotEnoughFunds,
    TransferError,
    //SomeError,
}

pub type ProposalId = u32;

#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
///Proposal result in a form (number of voters, For votes, Against votes)
pub struct ProposalResult(pub Balance, pub Balance, pub Balance);


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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub enum Status {
    Active,
    Archived,
    Executed,
    Rejected,
    Pending,
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
    pub status: Status,
    pub private_voting: bool,
    pub account_to: Option<AccountId>,
    pub amount: Option<Balance>,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
///Struct representing a single proposal's data
pub struct TimePeriod {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
}

impl Default for TimePeriod {
    fn default() -> Self { 
        Self { days: 0, hours: 0, minutes: 0 }
    }
}

impl TimePeriod {
    pub fn to_timestamp(&self) -> u64 {
        self.days * ONE_DAY + self.hours * ONE_HOUR + self.minutes * ONE_MINUTE
    }
}

pub const ONE_MINUTE: u64 = 60 * 1000;
pub const ONE_HOUR: u64 = 60 * ONE_MINUTE;
pub const ONE_DAY: u64 = 24 * ONE_HOUR;