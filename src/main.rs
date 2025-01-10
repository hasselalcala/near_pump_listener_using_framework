pub mod cli;
pub mod constants;
pub mod database;
pub mod models;
pub mod websocket;

pub use cli::*;
pub use constants::*;
pub use database::*;
pub use models::*;
pub use websocket::*;

use clap::Parser;
use dotenv::dotenv;
use near_event_listener::NearEventListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let cli = Cli::parse();

    let rpc_url = match cli.network {
        Networks::Mainnet => NEAR_RPC_MAINNET,
        Networks::Testnet => NEAR_RPC_TESTNET,
    };

    let db = start_db("DATABASE_URL").await?;

    let (ws_sender, ws_server) = start_websocket_server(8080).await;

    tokio::spawn(ws_server);

    let mut listener = NearEventListener::builder(rpc_url)
        .account_id(ACCOUNT_TO_LISTEN)
        .method_name(FUNCTION_TO_LISTEN)
        .last_processed_block(184520318)
        .build()?;

    listener
        .start(move |event_log| {
            let db = db.clone();
            let ws_sender = ws_sender.clone();
            tokio::spawn(async move {
                println!("Event received: {:?}", event_log);

                // Accumulate logs
                let mut logs = Vec::new();
                logs.push(event_log.clone());

                // Check if we have a register_token event
                if let Some(register_token_log) =
                    logs.iter().find(|log| log.event == "register_token")
                {
                    // Process the register_token event
                    if let Err(e) =
                        insert_token(&db, register_token_log.clone(), &ws_sender.clone()).await
                    {
                        eprintln!("Error inserting token: {}", e);
                    }
                    match get_tokens(&db).await {
                        Ok(tokens) => println!("Tokens: {:#?}", tokens),
                        Err(e) => eprintln!("Error getting tokens: {}", e),
                    }
                    // Clear logs after processing
                    logs.clear();
                }
            });
        })
        .await?;

    Ok(())
}
