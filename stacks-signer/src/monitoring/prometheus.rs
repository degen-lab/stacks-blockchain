// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020-2024 Stacks Open Internet Foundation
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use lazy_static::lazy_static;
use prometheus::{
    gather, opts, register_int_counter, register_int_counter_vec, register_int_gauge, Encoder,
    IntCounter, IntCounterVec, IntGauge, TextEncoder,
};

lazy_static! {
    pub static ref STACKS_TIP_HEIGHT_GUAGE: IntGauge = register_int_gauge!(opts!(
        "stacks_signer_stacks_node_height",
        "The current height of the Stacks node"
    ))
    .unwrap();
    pub static ref BLOCK_VALIDATION_RESPONSES: IntCounterVec = register_int_counter_vec!(
        "stacks_signer_block_validation_responses",
        "The number of block validation responses. `response_type` is either 'accepted' or 'rejected'",
        &["response_type"]
    )
    .unwrap();
    pub static ref BLOCK_RESPONSES_SENT: IntCounterVec = register_int_counter_vec!(
        "stacks_signer_block_responses_sent",
        "The number of block responses sent. `response_type` is either 'accepted' or 'rejected'",
        &["response_type"]
    )
    .unwrap();
    pub static ref SIGNER_INBOUND_MESSAGES: IntCounter = register_int_counter!(opts!(
        "stacks_signer_inbound_messages",
        "The number of inbound messages received by the signer"
    ))
    .unwrap();
    pub static ref COORDINATOR_INBOUND_MESSAGES: IntCounter = register_int_counter!(opts!(
        "stacks_signer_coordinator_inbound_messages",
        "The number of inbound messages received as a coordinator"
    ))
    .unwrap();
    pub static ref INBOUND_PACKETS_RECEIVED: IntCounter = register_int_counter!(opts!(
        "stacks_signer_inbound_packets_received",
        "The number of inbound packets received by the signer"
    ))
    .unwrap();
    pub static ref COMMANDS_PROCESSED: IntCounterVec = register_int_counter_vec!(
        "stacks_signer_commands_processed",
        "The number of commands processed by the signer",
        &["command_type"]
    )
    .unwrap();
    pub static ref DGK_VOTES_SUBMITTED: IntCounter = register_int_counter!(opts!(
        "stacks_signer_dgk_votes_submitted",
        "The number of DGK votes submitted by the signer"
    ))
    .unwrap();
    pub static ref OPERATION_RESULTS: IntCounterVec = register_int_counter_vec!(
        "stacks_signer_operation_results_dkg",
        "The number of DKG operation results",
        &["operation_type"]
    )
    .unwrap();
    pub static ref BLOCK_PROPOSALS_RECEIVED: IntCounter = register_int_counter!(opts!(
        "stacks_signer_block_proposals_received",
        "The number of block proposals received by the signer"
    ))
    .unwrap();
    pub static ref CURRENT_REWARD_CYCLE: IntGauge = register_int_gauge!(opts!(
        "stacks_signer_current_reward_cycle",
        "The current reward cycle"
    )).unwrap();
    pub static ref SIGNER_STX_BALANCE: IntGauge = register_int_gauge!(opts!(
        "stacks_signer_stx_balance",
        "The current STX balance of the signer"
    )).unwrap();
    pub static ref SIGNER_NONCE: IntGauge = register_int_gauge!(opts!(
        "stacks_signer_nonce",
        "The current nonce of the signer"
    )).unwrap();
}

pub fn gather_metrics_string() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metrics_families = gather();
    encoder.encode(&metrics_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
