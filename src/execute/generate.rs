use crate::{error::ContractError, msg::GenerateMsg, state::ExecuteContext};
use cosmwasm_std::{attr, Response};
// use crate::randomness::execute_job;

pub fn exec_generate(
    _ctx: ExecuteContext,
    _msg: GenerateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes(vec![attr("action", "generate")]))
}
