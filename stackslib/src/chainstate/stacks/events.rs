use crate::burnchains::Txid;
use crate::chainstate::burn::operations::BlockstackOperationType;
use crate::chainstate::nakamoto::NakamotoBlock;
use crate::chainstate::stacks::StacksBlock;
use crate::chainstate::stacks::StacksMicroblockHeader;
use crate::chainstate::stacks::StacksTransaction;
use clarity::vm::analysis::ContractAnalysis;
use clarity::vm::costs::ExecutionCost;
pub use clarity::vm::events::StacksTransactionEvent;
use clarity::vm::types::{
    AssetIdentifier, PrincipalData, QualifiedContractIdentifier, StandardPrincipalData, Value,
};
use stacks_common::codec::StacksMessageCodec;
use stacks_common::types::chainstate::BlockHeaderHash;
use stacks_common::types::chainstate::StacksAddress;
use stacks_common::util::hash::to_hex;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionOrigin {
    Stacks(StacksTransaction),
    Burn(BlockstackOperationType),
}

impl From<StacksTransaction> for TransactionOrigin {
    fn from(o: StacksTransaction) -> TransactionOrigin {
        TransactionOrigin::Stacks(o)
    }
}

impl TransactionOrigin {
    pub fn txid(&self) -> Txid {
        match self {
            TransactionOrigin::Burn(op) => op.txid(),
            TransactionOrigin::Stacks(tx) => tx.txid(),
        }
    }
    /// Serialize this origin type to a string that can be stored in
    ///  a database
    pub fn serialize_to_dbstring(&self) -> String {
        match self {
            TransactionOrigin::Burn(op) => format!("BTC({})", op.txid()),
            TransactionOrigin::Stacks(tx) => to_hex(&tx.serialize_to_vec()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StacksTransactionReceipt {
    pub transaction: TransactionOrigin,
    pub events: Vec<StacksTransactionEvent>,
    pub post_condition_aborted: bool,
    pub result: Value,
    pub stx_burned: u128,
    pub contract_analysis: Option<ContractAnalysis>,
    pub execution_cost: ExecutionCost,
    pub microblock_header: Option<StacksMicroblockHeader>,
    pub tx_index: u32,
    /// This is really a string-formatted CheckError (which can't be clone()'ed)
    pub vm_error: Option<String>,
}

#[derive(Clone)]
pub struct StacksBlockEventData {
    pub block_hash: BlockHeaderHash,
    pub parent_block_hash: BlockHeaderHash,
    pub parent_microblock_hash: BlockHeaderHash,
    pub parent_microblock_sequence: u16,
}

impl From<StacksBlock> for StacksBlockEventData {
    fn from(block: StacksBlock) -> StacksBlockEventData {
        StacksBlockEventData {
            block_hash: block.block_hash(),
            parent_block_hash: block.header.parent_block,
            parent_microblock_hash: block.header.parent_microblock,
            parent_microblock_sequence: block.header.parent_microblock_sequence,
        }
    }
}

impl From<(NakamotoBlock, BlockHeaderHash)> for StacksBlockEventData {
    fn from(block: (NakamotoBlock, BlockHeaderHash)) -> StacksBlockEventData {
        StacksBlockEventData {
            block_hash: block.0.header.block_hash(),
            parent_block_hash: block.1,
            parent_microblock_hash: BlockHeaderHash([0u8; 32]),
            parent_microblock_sequence: 0,
        }
    }
}
