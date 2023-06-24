mod bindings;
mod bundler;
mod utils;

use anyhow;
use anyhow::Context;
use env_logger::Env;
use ethers::{
    providers::{Provider, Ws},
    types::{H160, U256},
};
use std::env;
use std::panic;
use std::sync::Arc;
use tokio;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // generate bindings
    // utils::generate_bindings().unwrap();

    dotenv().ok();
    let goerli_url = env::var("WSS_RPC_GOERLI").context("WSS_RPC_GOERLI not set")?;
    let mumbai_url = env::var("WSS_RPC_MUMBAI").context("WSS_RPC_MUMBAI not set")?;
    
    let goerli_provider = Arc::new(
        Provider::<Ws>::connect(goerli_url)
            .await
            .ok()
            .ok_or(anyhow::anyhow!("Error connecting to Goerli"))?,
    );

    log::info!("Connected to Goerli");

    let mumbai_provider = Arc::new(
        Provider::<Ws>::connect(mumbai_url)
            .await
            .ok()
            .ok_or(anyhow::anyhow!("Error connecting to Mumbai"))?,
    );

    log::info!("Connected to Mumbai");

    let dummy = bundler::dumb_bundler::DumbBundler::new(
        goerli_provider.clone(),
        mumbai_provider.clone(),
        U256::max_value(),
        U256::max_value(),
    );

    log::info!("Created DumbBundler");

    // start the server
    let _ = std::thread::Builder::new()
        .name("dumb-bundler".to_string())
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_stack_size(128 * 1024 * 1024)
                .build()
                .expect("Failed to create Tokio runtime");

            let _ = rt.block_on(async {
                log::info!("Starting ERC-4337 AA Bundler");

                let task = tokio::spawn(async move {
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        println!("Hello, world!");
                    }
                });

                let _ = task.await;
            });
        })?
        .join()
        .unwrap_or_else(|e| panic::resume_unwind(e));

    Ok(())
}
