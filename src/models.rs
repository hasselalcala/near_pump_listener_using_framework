use near_sdk::json_types::Base64VecU8;
use near_sdk::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TokenDTO {
    pub owner_id: AccountId,
    pub total_supply: String,
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,
    pub image: String,
}
