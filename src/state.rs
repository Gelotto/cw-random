use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, DepsMut, Empty, Env, MessageInfo, Response};
use cw_storage_plus::{Item, Deque, Map};

use crate::{error::ContractError, msg::{InstantiateMsg, RequestMsg}};

pub const CONFIG: Item<Config> = Item::new("config");
pub const JOB_MAP: Map<u64, Deque<RequestMsg>> = Map::new("job_map");
pub const WHITELIST_MAP: Map<String, Empty> = Map::new("whitelist_addresses");

#[cw_serde]
pub struct Config {}

pub struct ExecuteContext<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

pub struct QueryContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}

impl ExecuteContext<'_> {
    /// Top-level initialization of contract state
    pub fn instantiate(
        &mut self,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let InstantiateMsg { config } = msg;
        CONFIG.save(self.deps.storage, &config)?;
        Ok(Response::new().add_attribute("action", "instantiate"))
    }
}
