use crate::{error::ContractError, msg::RequestMsg, state::ExecuteContext, state::JOB_MAP, state::OPERATOR};
use cosmwasm_std::{attr, Response};

pub fn exec_request(
    ctx: ExecuteContext,
    req_msg: RequestMsg,
) -> Result<Response, ContractError> {

    // TODO: funds handle part pay for each job?
    // check if the request msg height is valid
    let current_height = ctx.env.block.height;
    if current_height >= req_msg.height.into(){
        return Err(ContractError::InvalidHeight {
            current_height,
            requested_height: req_msg.height.into(),
        });
    }
    let requester_address = ctx.info.sender;
    // check if the address is th operator
    let contained_address = OPERATOR.may_load(ctx.deps.storage)?;
    if contained_address.is_none() || contained_address.unwrap() != requester_address {
        return Err(ContractError::NotAuthorized {
            reason: format!("Address {} is not authorized to request randomness", requester_address),
        });
    }
    // check if the request msg has at least one job
    if req_msg.jobs.is_empty() {
        return Err(ContractError::EmptyRequest {
            reason: "Request must have at least one job".to_string(),
        });
    }

    // check if the request msg has the recipients set, if not set requester address as the recipient
    let recipients = match req_msg.recipients {
        Some(recipients) => recipients,
        None => vec![requester_address],
    };

    let new_req_msg = RequestMsg {
        height: req_msg.height,
        recipients: Some(recipients),
        jobs: req_msg.jobs,
    };
    // store the request msg in the job map
    JOB_MAP.update(ctx.deps.storage, new_req_msg.height.into(), |jobs:Option<Vec<RequestMsg>>| -> Result<_, ContractError> {
        match jobs {
            Some(mut jobs) => {
                // add the new request msg to the list of jobs
                jobs.push(new_req_msg);
                Ok(jobs)
            }
            None => {
                let mut jobs = Vec::new();
                jobs.push(new_req_msg);
                Ok(jobs)
            }
        }
    })?;

    Ok(Response::new().add_attributes(vec![attr("action", "request")]))
}
