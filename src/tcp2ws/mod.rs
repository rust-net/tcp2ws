pub mod config;
pub mod service;
use futures_util::Future;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use crate::i;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

pub fn test() {
    run(async {
        i!("TEST...");
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        i!("TEST OK...");
    });
}

pub fn run<R>(future: impl Future<Output = R>) -> R {
    RUNTIME.block_on(future)
}