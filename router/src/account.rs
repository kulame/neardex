use crate::*;
use near_sdk::collections::UnorderedMap;
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
}
