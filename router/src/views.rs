use crate::*;
use near_sdk::near_bindgen;

#[near_bindgen]
impl Contract {
    pub fn get_user_tokens(&self, account_id: ValidAccountId) -> Vec<AccountId> {
        self.internal_get_account(account_id.as_ref())
            .map(|x| x.get_tokens())
            .unwrap_or_default()
    }
}
