use cosmwasm_std::{StdResult, from_slice};
use serde::de::DeserializeOwned;



pub(crate) fn may_deserialize<T: DeserializeOwned>(
    value: &Option<Vec<u8>>,
) -> StdResult<Option<T>> {
    match value {
        Some(data) => Ok(Some(from_slice(data)?)),
        None => Ok(None),
    }
}
