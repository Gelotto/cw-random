use crate::{error::ContractError, msg::RequestMsg, state::ExecuteContext, state::JOB_MAP, state::WHITELIST_MAP};
use cosmwasm_std::{attr, Response};

pub fn exec_request(
    ctx: ExecuteContext,
    req_msg: RequestMsg,
) -> Result<Response, ContractError> {

    // check if the request msg height is valid
    let current_height = ctx.env.block.height;
    if current_height >= req_msg.height.into(){
        return Err(ContractError::InvalidHeight {
            current_height,
            requested_height: req_msg.height.into(),
        });
    }
    // check if the address is whitelisted (this means that the address is authorized to request randomness for itself and automatically for other addresses)
    let requester_address = ctx.info.sender;
    // check if the address is not in the WHITELIST_MAP
    let contained_address = WHITELIST_MAP.may_load(ctx.deps.storage, requester_address.to_string())?;
    if contained_address.is_none() {
        return Err(ContractError::NotAuthorized {
            reason: format!("Address {} is not authorized to request randomness", requester_address),
        });
    }
    

    Ok(Response::new().add_attributes(vec![attr("action", "request")]))
}
