use crate::*;
use crate::{algorithm::Bft, error::BftError};

use crossbeam::crossbeam_channel::{unbounded, Sender};

#[cfg(feature = "message_cache")]
use std::collections::HashMap;

/// Results of Bft actuator.
pub type Result<T> = ::std::result::Result<T, BftError>;

/// A Bft Actuator
pub struct BftActuator(Sender<BftMsg>);

impl BftActuator {
    /// A function to create a new Bft actuator.

    pub fn new<T: BftSupport + Send + 'static>(support: T, address: Address) -> Self {
        let (sender, internal_receiver) = unbounded();
        Bft::start(internal_receiver, support, address);
        BftActuator(sender)
    }

    /// A function to send proposal.
    pub fn send_proposal(&self, proposal: Proposal) -> Result<()> {
        // check lock round and lock votes
        if proposal.lock_round.is_some() && proposal.lock_votes.is_empty() {
            return Err(BftError::ProposalIllegal(proposal.height, proposal.round));
        }
        self.0
            .send(BftMsg::Proposal(proposal))
            .map_err(|_| BftError::SendProposalErr)
    }

    /// A function to send vote.
    pub fn send_vote(&self, vote: Vote) -> Result<()> {
        self.0
            .send(BftMsg::Vote(vote))
            .map_err(|_| BftError::SendVoteErr)
    }

    /// A function to send status.
    pub fn send_status(&self, status: Status) -> Result<()> {
        self.0
            .send(BftMsg::Status(status))
            .map_err(|_| BftError::SendVoteErr)
    }

    /// A function to send command
    pub fn send_command(&self, cmd: BftMsg) -> Result<()> {
        match cmd {
            BftMsg::Pause => Ok(cmd),
            BftMsg::Start => Ok(cmd),
            _ => Err(BftError::MsgTypeErr),
        }
        .and_then(|cmd| self.0.send(cmd).map_err(|_| BftError::SendCmdErr))
    }
}
