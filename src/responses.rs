use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

use crate::state::Config;

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct RequestResponse {
    /// ID of pending request
    pub id: Uint64,
}
