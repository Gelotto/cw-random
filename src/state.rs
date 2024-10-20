use std::hash::{DefaultHasher, Hash, Hasher};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, DepsMut, Empty, Env, MessageInfo, Response};
use cw_storage_plus::{Item, Map};
use sha2::{Digest, Sha256};

use crate::{error::ContractError, msg::{InstantiateMsg, Job, RequestMsg}};

pub const CONFIG: Item<Config> = Item::new("config");
pub const JOB_MAP: Map<u64, Vec<RequestMsg>> = Map::new("job_map");
pub const OPERATOR: Item<Addr> = Item::new("operator");
pub const SEED: Item<SeedCW> = Item::new("seed");

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct SeedCW {
    pub seed: String,
    seed_u64: u64,
}

pub struct ExecuteContext<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

pub struct QueryContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
pub fn string_to_u64(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

impl SeedCW {

    pub fn new(seed: String) -> Self {
        let seed_u64 = string_to_u64(&seed);
        SeedCW { seed, seed_u64 }
    }

    pub fn update_seed(&mut self, env: &Env) {
        let mut hasher = Sha256::new();
        hasher.update(&self.seed);
        hasher.update(format!("{:?}", env.block.height));
        hasher.update(format!("{:?}", env.block.time.nanos()));
        self.seed = format!("{:?}", hasher.finalize());
        self.seed_u64 = string_to_u64(&self.seed);
    }

    pub fn get_u64_seed(&self) -> u64 {
        self.seed_u64
    }

}

impl ExecuteContext<'_> {
    /// Top-level initialization of contract state
    pub fn instantiate(
        &mut self,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let InstantiateMsg { config, operator } = msg;
        CONFIG.save(self.deps.storage, &config)?;
        let operator = operator.unwrap_or_else(|| self.info.sender.clone());
        OPERATOR.save(self.deps.storage, &operator)?;
        let mut seed = SeedCW::new("".to_string());
        seed.update_seed(&self.env);
        SEED.save(self.deps.storage, &seed)?;
        Ok(Response::new().add_attribute("action", "instantiate").add_attribute("operator", &operator))
    }
}