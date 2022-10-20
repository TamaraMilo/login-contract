use cosmwasm_std::{Storage, to_vec};

use crate::{type_helpers::may_deserialize, ContractError};

pub struct User;

impl User 
{
    pub fn save_user(username: String, addr: String, storage: &mut dyn Storage) -> Result<(), ContractError >
    {
        let user_exist = Self::load_user(username.clone(),storage );
        if user_exist != None{
            return Err(ContractError::UserError { val: "User already exists".to_string() }); 
        }
        let key = username.as_bytes();
        storage.set(key,&to_vec( &addr)?);
        Ok(())

        
    }
    pub fn load_user(username: String, storage: &dyn Storage) -> Option<String>
    {
        let key  = username.as_bytes();
        let user = storage.get(key);
        if user == None{
            return None;
        }
        let user :Option<String>= may_deserialize(&user).unwrap();
        user
    }
}