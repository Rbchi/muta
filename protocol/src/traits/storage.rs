use async_trait::async_trait;
use derive_more::Display;

use crate::codec::ProtocolCodec;
use crate::traits::Context;
use crate::types::block::{Block, Proof};
use crate::types::receipt::Receipt;
use crate::types::{Hash, SignedTransaction};
use crate::{Bytes, ProtocolResult};

#[derive(Debug, Copy, Clone, Display)]
pub enum StorageCategory {
    Block,
    Receipt,
    SignedTransaction,
    Wal,
}

pub trait StorageSchema {
    type Key: ProtocolCodec + Send;
    type Value: ProtocolCodec + Send;

    fn category() -> StorageCategory;
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn insert_transactions(
        &self,
        ctx: Context,
        signed_txs: Vec<SignedTransaction>,
    ) -> ProtocolResult<()>;

    async fn insert_block(&self, ctx: Context, block: Block) -> ProtocolResult<()>;

    async fn insert_receipts(&self, ctx: Context, receipts: Vec<Receipt>) -> ProtocolResult<()>;

    async fn update_latest_proof(&self, ctx: Context, proof: Proof) -> ProtocolResult<()>;

    async fn get_transaction_by_hash(
        &self,
        ctx: Context,
        tx_hash: Hash,
    ) -> ProtocolResult<SignedTransaction>;

    async fn get_transactions(
        &self,
        ctx: Context,
        hashes: Vec<Hash>,
    ) -> ProtocolResult<Vec<SignedTransaction>>;

    async fn get_latest_block(&self, ctx: Context) -> ProtocolResult<Block>;

    async fn get_block_by_height(&self, ctx: Context, height: u64) -> ProtocolResult<Block>;

    async fn get_block_by_hash(&self, ctx: Context, block_hash: Hash) -> ProtocolResult<Block>;

    async fn get_receipt(&self, ctx: Context, hash: Hash) -> ProtocolResult<Receipt>;

    async fn get_receipts(&self, ctx: Context, hash: Vec<Hash>) -> ProtocolResult<Vec<Receipt>>;

    async fn get_latest_proof(&self, ctx: Context) -> ProtocolResult<Proof>;

    async fn update_overlord_wal(&self, ctx: Context, info: Bytes) -> ProtocolResult<()>;

    async fn load_overlord_wal(&self, ctx: Context) -> ProtocolResult<Bytes>;
}

pub enum StorageBatchModify<S: StorageSchema> {
    Remove,
    Insert(<S as StorageSchema>::Value),
}

#[async_trait]
pub trait StorageAdapter: Send + Sync {
    async fn insert<S: StorageSchema>(
        &self,
        key: <S as StorageSchema>::Key,
        val: <S as StorageSchema>::Value,
    ) -> ProtocolResult<()>;

    async fn get<S: StorageSchema>(
        &self,
        key: <S as StorageSchema>::Key,
    ) -> ProtocolResult<Option<<S as StorageSchema>::Value>>;

    async fn get_batch<S: StorageSchema>(
        &self,
        keys: Vec<<S as StorageSchema>::Key>,
    ) -> ProtocolResult<Vec<Option<<S as StorageSchema>::Value>>> {
        let mut vec = Vec::new();

        for key in keys {
            vec.push(self.get::<S>(key).await?);
        }

        Ok(vec)
    }

    async fn remove<S: StorageSchema>(&self, key: <S as StorageSchema>::Key) -> ProtocolResult<()>;

    async fn contains<S: StorageSchema>(
        &self,
        key: <S as StorageSchema>::Key,
    ) -> ProtocolResult<bool>;

    async fn batch_modify<S: StorageSchema>(
        &self,
        keys: Vec<<S as StorageSchema>::Key>,
        vals: Vec<StorageBatchModify<S>>,
    ) -> ProtocolResult<()>;
}
