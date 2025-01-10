use crate::models::TokenDTO;
use crate::websocket::WsSender;

use base64::{engine::general_purpose, Engine as _};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use near_event_listener::EventLog;
use std::env;
use std::sync::Arc;
use tokio_postgres::NoTls;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn start_db(
    db_url: &str,
) -> Result<Arc<DbPool>, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var(db_url).expect("Variable not found");
    let db = Arc::new(init_db_pool(&url).await?);

    Ok(db)
}

pub async fn init_db_pool(
    db_url: &str,
) -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    let manager = PostgresConnectionManager::new_from_stringlike(db_url, NoTls)?;
    let pool = Pool::builder().build(manager).await?;

    {
        let conn = pool.get().await?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS block_height (
                key TEXT PRIMARY KEY,
                height BIGINT NOT NULL
            )",
            &[],
        )
        .await?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tokens (
    id SERIAL PRIMARY KEY,
    owner_id TEXT NOT NULL,
    total_supply TEXT NOT NULL,
    spec TEXT NOT NULL,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    icon TEXT,
    reference TEXT,
    reference_hash TEXT,
    decimals SMALLINT NOT NULL,
    image TEXT NOT NULL,
    description TEXT NOT NULL,
    auction_duration TEXT NOT NULL,
    min_buy_amount TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
)",
            &[],
        )
        .await?;
    }

    Ok(pool)
}

pub async fn insert_token(
    pool: &DbPool,
    event: EventLog,
    ws_sender: &WsSender,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = pool.get().await?;

    let token_data = event
        .data
        .as_array()
        .ok_or("Event data is not an array")?
        .first()
        .ok_or("Event data array is empty")?;

    conn.execute(
        "INSERT INTO tokens (
            owner_id, total_supply, spec, name, symbol, 
            icon, reference, reference_hash, decimals, image,
            description, auction_duration, min_buy_amount
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        &[
            &token_data["owner_id"]
                .as_str()
                .ok_or("owner_id not found")?,
            &token_data["total_supply"]
                .as_str()
                .ok_or("total_supply not found")?,
            &token_data["spec"].as_str().ok_or("spec not found")?,
            &token_data["name"].as_str().ok_or("name not found")?,
            &token_data["symbol"].as_str().ok_or("symbol not found")?,
            &token_data["icon"].as_str().ok_or("icon not found")?,
            &token_data["reference"].as_str(),
            &token_data["reference_hash"].as_str(),
            &(token_data["decimals"]
                .as_i64()
                .ok_or("decimals not found")? as i16),
            &token_data["image"].as_str().ok_or("image not found")?,
            &token_data["description"]
                .as_str()
                .ok_or("description not found")?,
            &token_data["auction_duration"]
                .as_str()
                .ok_or("auction_duration not found")?,
            &token_data["min_buy_amount"]
                .as_str()
                .ok_or("min_buy_amount not found")?,
        ],
    )
    .await?;

    let token = TokenDTO {
        owner_id: token_data["owner_id"].as_str().unwrap().parse().unwrap(),
        total_supply: token_data["total_supply"].as_str().unwrap().to_string(),
        spec: token_data["spec"].as_str().unwrap().to_string(),
        name: token_data["name"].as_str().unwrap().to_string(),
        symbol: token_data["symbol"].as_str().unwrap().to_string(),
        icon: token_data["icon"].as_str().map(|s| s.to_string()),
        reference: token_data["reference"].as_str().map(|s| s.to_string()),
        reference_hash: token_data["reference_hash"]
            .as_str()
            .map(|s| general_purpose::STANDARD.decode(s).unwrap().into()),
        decimals: token_data["decimals"].as_i64().unwrap() as u8,
        image: token_data["image"].as_str().unwrap().to_string(),
    };

    let _ = ws_sender.send(token.clone());
    Ok(())
}

pub async fn get_tokens(
    pool: &DbPool,
) -> Result<Vec<TokenDTO>, Box<dyn std::error::Error + Send + Sync>> {
    let conn = pool.get().await?;
    let rows = conn
        .query("SELECT * FROM tokens ORDER BY created_at DESC", &[])
        .await?;

    let tokens: Vec<TokenDTO> = rows
        .into_iter()
        .map(|row| TokenDTO {
            owner_id: row.get::<_, String>("owner_id").parse().unwrap(),
            total_supply: row
                .get::<_, String>("total_supply")
                .trim_matches('"')
                .to_string(),
            spec: row.get("spec"),
            name: row.get("name"),
            symbol: row.get("symbol"),
            icon: row.get("icon"),
            reference: row.get("reference"),
            reference_hash: row
                .get::<_, Option<String>>("reference_hash")
                .map(|s| general_purpose::STANDARD.decode(s).ok())
                .flatten()
                .map(|v| v.into()),
            decimals: row.get::<_, i16>("decimals") as u8,
            image: row.get("image"),
        })
        .collect();

    Ok(tokens)
}
