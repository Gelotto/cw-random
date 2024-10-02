use crate::{error::ContractError, msg::RequestMsg, state::ExecuteContext};
use cosmwasm_std::{attr, Response};

pub fn exec_request(
    _ctx: ExecuteContext,
    _msg: RequestMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes(vec![attr("action", "request")]))
}
