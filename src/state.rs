use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub wefund: Addr,
    pub anchor_market: Addr,
    pub aust_token: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BackerState{
    pub backer_wallet: String,
    pub ust_amount: Coin,
    pub aust_amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectState{
    pub project_id: Uint128,
    pub project_name: String,
    pub project_wallet: String,
    pub project_collected: Uint128,
    pub creator_wallet: String,
    pub project_website: String,
    pub project_about: String,
    pub project_email: String,
    pub project_ecosystem: String,
    pub project_category: String,
    pub project_needback: bool,
    pub project_done: Uint128, //0:backing 1:done 2:fail
    pub backer_states:Vec<BackerState>,
}
pub const PROJECT_SEQ: Item<Uint128> = Item::new("prj_seq");
pub const PROJECTSTATES: Map<U128Key, ProjectState> = Map::new("prj");


pub fn save_projectstate(deps: DepsMut, _prj: &ProjectState) 
-> StdResult<()> {
    // increment id if exists, or return 1
    let id = PROJECT_SEQ.load(deps.storage)?;
    let id = id.checked_add(Uint128::new(1))?;
    PROJECT_SEQ.save(deps.storage, &id)?;

    // save pot with id
    let mut project = _prj.clone();
    project.project_id = id.clone();
    PROJECTSTATES.save(deps.storage, id.u128().into(), &project)
}
