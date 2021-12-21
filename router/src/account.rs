use crate::*;
use near_sdk::{assert_one_yocto, collections::UnorderedMap};
use near_sdk::{env, AccountId, Balance, StorageUsage};
pub const UNIT_STORAGE: StorageUsage = 256;

impl Account {
    pub fn new(account_id: &AccountId) -> Self {
        Self {
            near_amount: 0,
            tokens: UnorderedMap::new(StorageKey::AccountTokens {
                account_id: account_id.clone(),
            }),
            storage_used: 0,
            account_id: account_id.clone(),
        }
    }

    pub fn storage_usage(&self) -> Balance {
        (UNIT_STORAGE + self.tokens.len() as u64 * UNIT_STORAGE) as u128 * env::storage_byte_cost()
    }

    pub fn storage_available(&self) -> Balance {
        let locked = self.storage_usage();
        if self.near_amount > locked {
            self.near_amount - locked
        } else {
            0
        }
    }

    pub fn min_storage_usage() -> Balance {
        UNIT_STORAGE as Balance * env::storage_byte_cost()
    }

    pub fn assert_storage_usage(&self) {
        env::log(self.storage_usage().to_string().as_bytes());
        env::log(self.near_amount.to_string().as_bytes());
        assert!(
            self.storage_usage() <= self.near_amount,
            "{}",
            ERR11_INSUFFICIENT_STORAGE
        )
    }

    pub fn get_tokens(&self) -> Vec<AccountId> {
        self.tokens.keys().collect()
    }

    pub fn get_balance(&self, token_id: &AccountId) -> Option<Balance> {
        if let Some(token_balance) = self.tokens.get(token_id) {
            Some(token_balance)
        } else {
            None
        }
    }

    pub fn register(&mut self, token_ids: &Vec<ValidAccountId>) {
        for token_id in token_ids {
            let t = token_id.as_ref();
            if self.get_balance(t).is_none() {
                self.tokens.insert(t, &0);
            }
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn register_tokens(&mut self, token_ids: Vec<ValidAccountId>) {
        assert_one_yocto();
        self.assert_contract_running();
        let sender_id = env::predecessor_account_id();
        let mut account = self
            .internal_get_account(&sender_id)
            .expect(ERR10_ACC_NOT_REGISTERED);
        account.register(&token_ids);
        self.internal_save_account(account);
    }
}
