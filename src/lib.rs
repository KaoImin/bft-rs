//! An efficent and stable Rust library of BFT protocol for distributed system.
//!
//!

#![deny(missing_docs)]

extern crate bincode;
#[macro_use]
extern crate crossbeam;
#[macro_use]
extern crate log;
extern crate lru_cache;
extern crate min_max_heap;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;

use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

/// Bft actuator.
pub mod actuator;
/// BFT state machine.
pub mod algorithm;
///
pub mod error;
/// BFT params include time interval and local address.
pub mod params;
/// BFT timer.
pub mod timer;
/// BFT vote set.
pub mod voteset;

/// Type for node address.
pub type Address = Vec<u8>;
/// Type for proposal.
pub type Target = Vec<u8>;

/// BFT input message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum BftMsg {
    /// Proposal message.
    Proposal(Proposal),
    /// Vote message.
    Vote(Vote),
    /// Feed messge, this is the proposal of the height.
    Feed(Feed),
    /// Verify response
    #[cfg(feature = "verify_req")]
    VerifyResp(VerifyResp),
    /// Status message, rich status.
    Status(Status),
    /// Commit message.
    Commit(Commit),
    /// Pause BFT state machine.
    Pause,
    /// Start running BFT state machine.
    Start,
}

/// Bft vote types.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VoteType {
    /// Vote type prevote.
    Prevote,
    /// Vote type precommit.
    Precommit,
}

impl ToJson for VoteType {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let t: u8;
        match self {
            VoteType::Prevote => t = 0,
            VoteType::Precommit => t = 1,
        }
        if t == 0 {
            d.insert("Prevote".to_string(), t.to_json());
        } else {
            d.insert("Precommit".to_string(), t.to_json());
        }
        Json::Object(d)
    }
}

/// Something need to be consensus in a round.
/// A `Proposal` includes `height`, `round`, `content`, `lock_round`, `lock_votes`
/// and `proposer`. `lock_round` and `lock_votes` are `Option`, means the PoLC of
/// the proposal. Therefore, these must have same variant of `Option`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    /// The height of proposal.
    pub height: u64,
    /// The round of proposal.
    pub round: u64,
    /// The proposal content.
    pub content: Target,
    /// A lock round of the proposal.
    pub lock_round: Option<u64>,
    /// The lock votes of the proposal.
    pub lock_votes: Option<Vec<Vote>>,
    /// The address of proposer.
    pub proposer: Address,
}

impl ToJson for Proposal {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("height".to_string(), self.height.to_json());
        d.insert("round".to_string(), self.round.to_json());
        d.insert("hash".to_string(), self.content.to_json());
        d.insert("lock round".to_string(), self.lock_round.to_json());
        d.insert("lock votes".to_string(), self.lock_votes.to_json());
        d.insert("proposer".to_string(), self.proposer.to_json());
        Json::Object(d)
    }
}

/// A PoLC.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LockStatus {
    /// The lock proposal
    pub proposal: Target,
    /// The lock round
    pub round: u64,
    /// The lock votes.
    pub votes: Vec<Vote>,
}

/// A vote to a proposal.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vote {
    /// Prevote vote or precommit vote
    pub vote_type: VoteType,
    /// The height of vote
    pub height: u64,
    /// The round of vote
    pub round: u64,
    /// The vote proposal
    pub proposal: Target,
    /// The address of voter
    pub voter: Address,
}

impl ToJson for Vote {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("vote type".to_string(), self.vote_type.to_json());
        d.insert("height".to_string(), self.height.to_json());
        d.insert("round".to_string(), self.round.to_json());
        d.insert("proposal".to_string(), self.proposal.to_json());
        d.insert("voter".to_string(), self.voter.to_json());
        Json::Object(d)
    }
}

/// A proposal for a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Feed {
    /// The height of the proposal.
    pub height: u64,
    /// A proposal.
    pub proposal: Target,
}

/// A result of a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// The height of result.
    pub height: u64,
    /// The round of result.
    pub round: u64,
    /// Consensus result
    pub proposal: Target,
    /// Votes for generate proof.
    pub lock_votes: Vec<Vote>,
    /// The node address.
    pub address: Address,
}

impl ToJson for Commit {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("height".to_string(), self.height.to_json());
        d.insert("round".to_string(), self.round.to_json());
        d.insert("proposal".to_string(), self.proposal.to_json());
        d.insert("lock votes".to_string(), self.lock_votes.to_json());
        d.insert("address".to_string(), self.address.to_json());
        Json::Object(d)
    }
}

/// Necessary messages for a height.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Status {
    /// The height of rich status.
    pub height: u64,
    /// The time interval of next height. If it is none, maintain the old interval.
    pub interval: Option<u64>,
    /// A new authority list for next height.
    pub authority_list: Vec<Address>,
}

/// A verify result of a proposal.
#[cfg(feature = "verify_req")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VerifyResp {
    /// The Response of proposal verify
    pub is_pass: bool,
    /// The verify proposal
    pub proposal: Target,
}
