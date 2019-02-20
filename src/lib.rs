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
#![allow(unused_imports)]
#![allow(unused_results)]
#![feature(try_from)]

#[macro_use]
extern crate bincode;
#[macro_use]
extern crate crossbeam;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate lru_cache;
extern crate min_max_heap;
#[macro_use]
extern crate serde_derive;

use algorithm::Step;

pub mod algorithm;
pub mod params;
pub mod timer;
pub mod voteset;
pub mod wal;

pub type Address = Vec<u8>;
pub type Target = Vec<u8>;

#[derive(Clone, Debug)]
pub enum BftMsg {
    Proposal(Proposal),
    Vote(Vote),
    Feed(Feed),
    Status(Status),
    Commit(Commit),
}

#[derive(Clone, Debug)]
pub struct Proposal {
    height: usize,
    round: usize,
    content: Target, // block hash
    lock_round: Option<usize>,
    lock_votes: Option<Vec<Vote>>,
    proposer: Address,
}

#[derive(Clone, Debug)]
pub struct LockStatus {
    proposal: Target, // block hash
    round: usize,
    votes: Vec<Vote>,
}

#[derive(Clone, Debug)]
pub struct Vote {
    vote_type: Step,
    height: usize,
    round: usize,
    proposal: Target, // block hash
    voter: Address,
}

#[derive(Clone, Debug)]
pub struct Feed {
    pub height: usize,
    pub proposal: Target, // block hash
}

#[derive(Clone, Debug)]
pub struct Commit {
    pub height: usize,
    pub proposal: Target, // block hash
    pub lock_votes: Vec<Vote>,
}

#[derive(Clone, Debug)]
pub struct Status {
    pub height: usize,
    pub interval: Option<u64>,
    pub authority_list: Vec<Address>,
}
