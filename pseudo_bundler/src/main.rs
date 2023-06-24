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
use tower::ServiceBuilder;
use std::future::pending;
use jsonrpsee::{
    server::ServerBuilder,
    Methods
};

const RPC_ENDPOINT: &str = "127.0.0.1:3000";

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
                    log::info!("Starting RPC server");

                    // let service = ServiceBuilder::new()
                    //             .layer(ProxyJsonRpcLayer::new(mumbai_url.clone()));
                    let server = ServerBuilder::new()
                                // .set_middleware(service)
                                .build(RPC_ENDPOINT)
                                .await
                                .map_err(|e| anyhow::anyhow!("Error starting server: {:?}", e))?;

                    let mut methods = Methods::new();

                    let _ = server.start(methods.clone())?;


                    Ok::<(), anyhow::Error>(())
                });
                let _ = task.await;

                let ctrl_c = tokio::signal::ctrl_c();
                tokio::select! {
                    _ = ctrl_c => {
                        println!("Ctrl+C received, shutting down");
                    }
                    else => {
                        println!("Server stopped unexpectedly");
                    }
                }

            });
        })?
        .join()
        .unwrap_or_else(|e| panic::resume_unwind(e));

    Ok(())
}
