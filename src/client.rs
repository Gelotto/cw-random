use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, Int128, Int256, Int64, StdError, SubMsg, Uint128, Uint256, Uint64, WasmMsg,
};

use crate::msg::RequestMsg;

#[cw_serde]
pub struct ReceiveRandomnessMsg {
    /// ID of originating request
    pub id: Uint64,
    /// Random values generated
    pub results: Vec<JobResult>,
}

#[cw_serde]
pub enum JobResult {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<Uint64>),
    U128(Vec<Uint128>),
    // U256(Vec<Uint256>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<Int64>),
    I128(Vec<Int128>),
    // I256(Vec<Int256>),
    Choice(Vec<String>),
}

/// Build a submsg that requests randomness
pub fn request_randomness_submsg(
    msg: RequestMsg,
    random_contract_addr: Addr,
    funds: Vec<Coin>,
    reply_id: u64,
) -> Result<SubMsg, StdError> {
    Ok(SubMsg::reply_on_success(
        WasmMsg::Execute {
            contract_addr: random_contract_addr.to_string(),
            msg: to_json_binary(&msg)?,
            funds,
        },
        reply_id,
    ))
}
