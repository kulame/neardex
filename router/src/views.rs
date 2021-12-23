use crate::*;
use near_sdk::near_bindgen;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub struct StorageState {
    pub deposit: U128,
    pub usage: U128,
}

#[near_bindgen]
impl Contract {
    pub fn get_user_tokens(&self, account_id: ValidAccountId) -> Vec<AccountId> {
        self.internal_get_account(account_id.as_ref())
            .map(|x| x.get_tokens())
            .unwrap_or_default()
    }

    pub fn get_user_storage_state(&self, account_id: ValidAccountId) -> Option<StorageState> {
        self.internal_get_account(account_id.as_ref())
            .map(|x| StorageState {
                deposit: U128(x.near_amount),
                usage: U128(x.storage_usage()),
            })
    }

    pub fn get_deposits(&self, account_id: ValidAccountId) -> HashMap<AccountId, U128> {
        let wrapped_account = self.internal_get_account(account_id.as_ref());
        if let Some(account) = wrapped_account {
            account
                .get_tokens()
                .iter()
                .map(|token| (token.clone(), U128(account.get_balance(token).unwrap())))
                .collect()
        } else {
            HashMap::new()
        }
    }
}
