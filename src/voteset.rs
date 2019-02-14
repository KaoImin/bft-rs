// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use super::{Address, Target, Vote, VoteType};
use lru_cache::LruCache;

use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Debug)]
pub struct VoteCollector {
    pub votes: LruCache<usize, RoundCollector>,
    pub prevote_count: HashMap<usize, usize>,
}

impl VoteCollector {
    pub fn new() -> Self {
        VoteCollector {
            votes: LruCache::new(16),
            prevote_count: HashMap::new(),
        }
    }

    pub fn add(&mut self, vote: Vote) -> bool {
        let height = vote.height;
        let round = vote.round;
        let vote_type = vote.vote_type;
        let sender = vote.voter;
        let vote = vote.proposal;

        if vote_type == VoteType::Prevote {
            if self.votes.contains_key(&height) {
                if self
                    .votes
                    .get_mut(&height)
                    .unwrap()
                    .add(round, vote_type, sender, vote)
                {
                    // update prevote count hashmap
                    let counter = self.prevote_count.entry(round).or_insert(0);
                    *counter += 1;
                    true
                } else {
                    // if add prevote fail, do not update prevote hashmap
                    false
                }
            } else {
                let mut round_votes = RoundCollector::new();
                round_votes.add(round, vote_type, sender, vote);
                self.votes.insert(height, round_votes);
                // update prevote count hashmap
                let counter = self.prevote_count.entry(round).or_insert(0);
                *counter += 1;
                true
            }
        } else if self.votes.contains_key(&height) {
            self.votes
                .get_mut(&height)
                .unwrap()
                .add(round, vote_type, sender, vote)
        } else {
            let mut round_votes = RoundCollector::new();
            round_votes.add(round, vote_type, sender, vote);
            self.votes.insert(height, round_votes);
            true
        }
    }

    pub fn get_voteset(
        &mut self,
        height: usize,
        round: usize,
        vote_type: VoteType,
    ) -> Option<VoteSet> {
        self.votes
            .get_mut(&height)
            .and_then(|rc| rc.get_voteset(round, vote_type))
    }

    pub fn clear_prevote_count(&mut self) {
        self.prevote_count.clear();
    }
}

// 1. sender's vote message  2. proposal's hash  3. count
#[derive(Clone, Debug)]
pub struct VoteSet {
    pub votes_by_sender: HashMap<Address, Target>,
    pub votes_by_proposal: HashMap<Target, usize>,
    pub count: usize,
}

impl VoteSet {
    pub fn new() -> Self {
        VoteSet {
            votes_by_sender: HashMap::new(),
            votes_by_proposal: HashMap::new(),
            count: 0,
        }
    }

    // just add, not check
    pub fn add(&mut self, sender: Address, vote: Target) -> bool {
        let mut is_add = false;
        self.votes_by_sender.entry(sender).or_insert_with(|| {
            is_add = true;
            vote.to_owned()
        });
        if is_add {
            self.count += 1;
            *self.votes_by_proposal.entry(vote).or_insert(0) += 1;
        }
        is_add
    }

    pub fn abstract_polc(
        &self,
        height: usize,
        round: usize,
        vote_type: VoteType,
        proposal: &Target,
    ) -> Vec<Vote> {
        // abstract the votes for the polc proposal into a vec
        let mut polc = Vec::new();
        for (address, vote_proposal) in &self.votes_by_sender {
            if vote_proposal == proposal {
                polc.push(Vote {
                    vote_type,
                    height,
                    round,
                    proposal: proposal.clone(),
                    voter: address.clone(),
                });
            }
        }
        polc
    }
}

// round -> step collector
#[derive(Debug)]
pub struct RoundCollector {
    pub round_votes: LruCache<usize, StepCollector>,
}

impl RoundCollector {
    pub fn new() -> Self {
        RoundCollector {
            round_votes: LruCache::new(16),
        }
    }

    pub fn add(
        &mut self,
        round: usize,
        vote_type: VoteType,
        sender: Address,
        vote: Target,
    ) -> bool {
        if self.round_votes.contains_key(&round) {
            self.round_votes
                .get_mut(&round)
                .unwrap()
                .add(vote_type, sender, vote)
        } else {
            let mut step_votes = StepCollector::new();
            step_votes.add(vote_type, sender, vote);
            self.round_votes.insert(round, step_votes);
            true
        }
    }

    pub fn get_voteset(&mut self, round: usize, vote_type: VoteType) -> Option<VoteSet> {
        self.round_votes
            .get_mut(&round)
            .and_then(|sc| sc.get_voteset(vote_type))
    }
}

// step -> voteset
#[derive(Debug, Default)]
pub struct StepCollector {
    pub step_votes: HashMap<VoteType, VoteSet>,
}

impl StepCollector {
    pub fn new() -> Self {
        StepCollector {
            step_votes: HashMap::new(),
        }
    }

    pub fn add(&mut self, vote_type: VoteType, sender: Address, vote: Target) -> bool {
        self.step_votes
            .entry(vote_type)
            .or_insert_with(VoteSet::new)
            .add(sender, vote)
    }

    pub fn get_voteset(&self, vote_type: VoteType) -> Option<VoteSet> {
        self.step_votes.get(&vote_type).cloned()
    }
}