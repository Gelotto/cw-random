use crate::{error::ContractError, msg::GenerateMsg, state::ExecuteContext};
use cosmwasm_std::{attr, Response};

pub fn exec_generate(
    _ctx: ExecuteContext,
    _msg: GenerateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes(vec![attr("action", "generate")]))
}
