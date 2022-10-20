use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAccount{
        username: String, 
        address: String
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AccountResponse)]
    AccountExist{ username: String}
}


#[cw_serde(Serialize)]
pub struct AccountResponse{
    pub account_exist: bool,
}