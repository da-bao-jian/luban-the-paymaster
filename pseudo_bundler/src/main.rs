mod bindings;
mod bundler;
mod utils;

use crate::bundler::dumb_bundler::EthApiServer;
use aa_bundler_primitives::{
    UserOperation, UserOperationByHash, UserOperationGasEstimation, UserOperationHash,
    UserOperationPartial, UserOperationReceipt, Wallet,
};
use anyhow::Context;
use dotenv::dotenv;
use env_logger::Env;
use ethers::{
    providers::{Provider, Ws},
    types::{H160, U256, U64},
};
use expanded_pathbuf::ExpandedPathBuf;
use jsonrpsee::server::ServerBuilder;
use reqwest;
use std::env;
use std::panic;
use std::str::FromStr;
use std::sync::Arc;
use tower::ServiceBuilder;
use warp::Filter;

const RPC_ENDPOINT: &str = "127.0.0.1:3001";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(
        Env::default().default_filter_or("info"), // .default_filter_or("trace")
    )
    .init();

    if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::fmt::init();
    }
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
        Provider::<Ws>::connect(mumbai_url.clone())
            .await
            .ok()
            .ok_or(anyhow::anyhow!("Error connecting to Mumbai"))?,
    );

    log::info!("Connected to Mumbai");

    let bundle_signer = env::var("FLASHBOTS_IDENTIFIER").unwrap_or_else(|e| {
        panic!("Please set the FLASHBOTS_IDENTIFIER environment variable");
    });

    let _wallet = env::var("PRIVATE_KEY_PATH").unwrap_or_else(|e| {
        panic!("Please set the PRIVATE_KEY_PATH environment variable");
    });
    let wallet = Wallet::from_file(
        ExpandedPathBuf::from_str(&_wallet.clone()).unwrap(),
        &U256::from(80001),
    )
    .unwrap();
    log::info!("{:?}", wallet.signer);

    let dummy = bundler::dumb_bundler::DumbBundler::new(
        goerli_provider.clone(),
        mumbai_provider.clone(),
        U256::max_value(),
        U256::max_value(),
        wallet,
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

                    let service = ServiceBuilder::new().layer(
                        bundler::middleware::ProxyJsonRpcLayer::new(mumbai_url.clone()),
                    );
                    let server = ServerBuilder::new()
                        .set_middleware(service)
                        .build(RPC_ENDPOINT)
                        .await
                        .map_err(|e| anyhow::anyhow!("Error starting server: {:?}", e))
                        .unwrap();

                    log::info!("Started RPC server at {}", server.local_addr().unwrap());

                    let _handle = server.start(dummy.into_rpc()).unwrap();

                    // Create a warp filter for CORS
                    let cors = warp::cors()
                        .allow_any_origin()
                        .allow_methods(vec!["POST", "GET"])
                        .allow_header("Content-Type");

                    // Create a warp filter that forwards requests to jsonrpsee server
                    let forward = warp::any()
                        .and(warp::body::json())
                        .and_then(move |body: serde_json::Value| async move {
                            // Convert the body to a string
                            let body_str = serde_json::to_string(&body).unwrap();

                            log::info!("Request to JSON-RPC server: {}", body_str);

                            // Create a new HTTP client
                            let client = reqwest::Client::new();

                            // Send the request to the jsonrpsee server
                            let res = client
                                .post("http://localhost:3001")
                                .header("Content-Type", "application/json")
                                .body(body_str)
                                .send()
                                .await
                                .unwrap();

                            // Convert the response back to JSON
                            let json: serde_json::Value = res.json().await.unwrap_or_else(|e| {
                                log::error!("Error converting response to JSON: {:?}", e);
                                serde_json::json!({
                                    "jsonrpc": "2.0",
                                    "error": {
                                        "code": -32603,
                                        "message": "Internal error"
                                    },
                                    "id": body["id"]
                                })
                            });
                            log::info!("Response from JSON-RPC server: {:?}", json);

                            Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&json))
                        })
                        .with(cors);

                    // Start the warp server
                    warp::serve(forward).run(([127, 0, 0, 1], 3002)).await;

                    loop {
                        let stopped = _handle.is_stopped();
                        log::info!("The server is running: {}", !stopped);
                        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    }

                    // Ok::<(), anyhow::Error>(())
                });
                let _ = task.await;
            });
            rt.block_on(async {
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
