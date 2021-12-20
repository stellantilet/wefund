use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Project id is already registerd")]
    AlreadyRegisteredProject {},

    #[error("Project id is not registerd yet")]
    NotRegisteredProject {},

    #[error("Need some coin")]
    NeedCoin{},

    #[error("Could not transfer")]
    COULDNOTTRANSFER{},

    #[error("Contract Address is alreay registered")]
    AlreadyRegisteredContract{},

    #[error("Not Found Available Project Contract Address")]
    NOTFOUNDAVAILABLEPROJECTCONTRACT{},
    
    #[error("Alreay enough collected")]
    AlreadyCollected{},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
