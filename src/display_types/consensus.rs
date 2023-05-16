/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert data types from hotstuff crate to a format which can be displayed on the terminal.
use serde::Serialize;
use crate::command::Base64String;

/// [QuorumCertificate] denotes a display_types equivalent of hotstuff_rs_types::QuorumCertificate.
#[derive(Serialize, Debug)]
pub struct QuorumCertificate {
    pub chain_id: u64,
    pub view: u64,
    pub block: Base64String,
    pub phase: Phase,
    pub signatures: SignatureSet,
}

impl From<hotstuff_rs::types::QuorumCertificate> for QuorumCertificate {
    fn from(qc: hotstuff_rs::types::QuorumCertificate) -> Self {
        Self {
            chain_id: qc.chain_id,
            view: qc.view,
            block: base64url::encode(qc.block),
            phase: From::<hotstuff_rs::types::Phase>::from(qc.phase),
            signatures: From::<hotstuff_rs::types::SignatureSet>::from(qc.signatures),
        }
    }
}

#[derive(Serialize, Debug)]
pub enum Phase {
    // ↓↓↓ For pipelined flow ↓↓↓ //   
    Generic,
    // ↓↓↓ For phased flow ↓↓↓ //
    Prepare, 
    // The inner view number is the view number of the *prepare* qc contained in the nudge which triggered the
    // vote containing this phase.
    Precommit(u64),
    // The inner view number is the view number of the *precommit* qc contained in the nudge which triggered the
    // vote containing this phase.
    Commit(u64),
}
impl From<hotstuff_rs::types::Phase> for Phase {
    fn from(phase: hotstuff_rs::types::Phase) -> Self {
        match phase {
            hotstuff_rs::types::Phase::Generic => Phase::Generic,
            hotstuff_rs::types::Phase::Prepare => Phase::Prepare,
            hotstuff_rs::types::Phase::Precommit(view_num) => Phase::Precommit(view_num),
            hotstuff_rs::types::Phase::Commit(view_num) => Phase::Commit(view_num),
        }
    }
}

/// [SignatureSet] denotes a display_types equivalent hotstuff_rs_types::SignatureSet.
#[derive(Serialize, Debug)]
pub struct SignatureSet {
    pub signatures: Vec<Option<Base64String>>,
}

impl From<hotstuff_rs::types::SignatureSet> for SignatureSet {
    fn from(sig: hotstuff_rs::types::SignatureSet) -> Self {
        let signatures: Vec<Option<Base64String>> = sig.iter().map(|s|{
            s.map(base64url::encode)
        }).collect();
        Self {
            signatures,
        }
    }
}