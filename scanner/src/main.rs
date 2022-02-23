pub mod range_scanner;

#[macro_use]
extern crate log;

mod error;
pub use error::*;

pub mod commands;
pub mod db;
pub mod rpc;
pub mod tx;

use clap::Parser;

use commands::Scanner;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    match commands::Scanner::parse() {
        Scanner::Load(load) => load.execute().await,
        Scanner::Scan(batch_scan) => batch_scan.execute().await,
    }
}
