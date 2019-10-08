#[cfg(feature = "ethereum")]
use super::super::redis_ethereum_ledger::EthereumLedgerRedisStore;
use super::redis_helpers::TestContext;
use crate::stores::redis_store_common::{EngineRedisStore, EngineRedisStoreBuilder};

use env_logger;
use futures::Future;
use tokio::runtime::Runtime;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref IDEMPOTENCY_KEY: String = String::from("abcd");
}

pub fn test_store() -> impl Future<Item = (EngineRedisStore, TestContext), Error = ()> {
    let context = TestContext::new();
    EngineRedisStoreBuilder::new(context.get_client_connection_info())
        .connect()
        .and_then(|store| Ok((store, context)))
}

#[cfg(feature = "ethereum")]
pub fn test_eth_store() -> impl Future<Item = (EthereumLedgerRedisStore, TestContext), Error = ()> {
    let context = TestContext::new();
    EngineRedisStoreBuilder::new(context.get_client_connection_info())
        .connect()
        .and_then(|redis_store| Ok((EthereumLedgerRedisStore::new(redis_store), context)))
}

pub fn block_on<F>(f: F) -> Result<F::Item, F::Error>
where
    F: Future + Send + 'static,
    F::Item: Send,
    F::Error: Send,
{
    // Only run one test at a time
    let _ = env_logger::try_init();
    let mut runtime = Runtime::new().unwrap();
    runtime.block_on(f)
}
