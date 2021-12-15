use crate::*;
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen};
use std::convert::TryInto;

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.assert_contract_running();
        let amount = env::attached_deposit();
        let account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(|| env::predecessor_account_id());
        let registration_only = registration_only.unwrap_or(false);
        let min_balance = self.storage_balance_bounds().min.0;
        let already_registered = self.accounts.contains_key(&account_id);
        if amount < min_balance && !already_registered {
            env::panic(format!("deposit is less than {}", min_balance).as_bytes());
        }
        if registration_only {
            if already_registered {
                log!("ERR_ACC_REGISTERED");
            }
        }

        StorageBalance {
            total: U128(1),
            available: U128(1),
        }
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        let account_id = env::predecessor_account_id();
        let id = "kulasama.near".try_into().unwrap();
        self.storage_balance_of(id).unwrap()
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        true
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: Account::min_storage_usage().into(),
            max: None,
        }
    }

    fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance> {
        self.internal_get_account(account_id.as_ref())
            .map(|account| StorageBalance {
                total: U128(account.near_amount),
                available: U128(account.storage_available()),
            })
    }
}
