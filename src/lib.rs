#[macro_use]
extern crate prettytable;

pub mod cmd;
pub mod format;
pub mod keypair;
pub mod memo;
pub mod mnemonic;
pub mod pwhash;
pub mod result;
pub mod staking;
pub mod traits;
pub mod wallet;

pub(crate) fn synchronize<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .expect("failed to create async runtime")
        .block_on(future)
}
