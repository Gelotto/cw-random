use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Int128, Int256, Int64, Uint128, Uint256, Uint64};

#[allow(unused_imports)]
use crate::{responses::ConfigResponse, state::Config};

#[cw_serde]
pub struct InstantiateMsg {
    pub config: Config,
    pub operator: Option<Addr>
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    SetConfig(Config),
    Request(RequestMsg),
    Generate(GenerateMsg),
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct RequestMsg {
    /// Block height to generate randomness and execute client's
    /// ReceiveRandomness callback.
    pub height: Uint64,
    /// List of recipients of randomness when generated, if different than the
    /// transaction sender. If defined, randomness will be sent ONLY to the
    /// specified addrs.
    pub recipients: Option<Vec<Addr>>,
    /// One or more random values to generate and return.
    pub jobs: Vec<Job>,
}

#[cw_serde]
pub enum Job {
    U8 {
        min: Option<u8>,
        max: Option<u8>,
        n: u16,
    },
    U16 {
        min: Option<u16>,
        max: Option<u16>,
        n: u16,
    },
    U32 {
        min: Option<u32>,
        max: Option<u32>,
        n: u16,
    },
    U64 {
        min: Option<Uint64>,
        max: Option<Uint64>,
        n: u16,
    },
    U128 {
        min: Option<Uint128>,
        max: Option<Uint128>,
        n: u16,
    },
    // U256 {
    //     min: Option<Uint256>,
    //     max: Option<Uint256>,
    //     n: u16,
    // },
    I8 {
        min: Option<i8>,
        max: Option<i8>,
        n: u16,
    },
    I16 {
        min: Option<i16>,
        max: Option<i16>,
        n: u16,
    },
    I32 {
        min: Option<i32>,
        max: Option<i32>,
        n: u16,
    },
    I64 {
        min: Option<Int64>,
        max: Option<Int64>,
        n: u16,
    },
    I128 {
        min: Option<Int128>,
        max: Option<Int128>,
        n: u16,
    },
    // I256 {
    //     min: Option<Int256>,
    //     max: Option<Int256>,
    //     n: u16,
    // },
    Choice {
        samples: Vec<String>,
        weights: Option<Vec<u32>>,
        with_replacement: bool,
        n: u16,
    },
}

#[cw_serde]
pub struct GenerateMsg {
    /// ID of the pending job to execute
    pub id: Uint64,
}
