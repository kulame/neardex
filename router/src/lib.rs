use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::AccountId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, near_bindgen, Balance, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult, StorageUsage,
};
use utils::{get_amount, get_pair_name};

use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
mod account;
mod errors;
mod storage_impl;
mod utils;

use crate::errors::*;
use crate::utils::quote;

near_sdk::setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Accounts,
    AccountTokens { account_id: AccountId },
}
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Account {
    pub account_id: AccountId,
    pub near_amount: Balance,
    pub tokens: UnorderedMap<AccountId, Balance>,
    pub storage_used: StorageUsage,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    factory: AccountId,
    accounts: LookupMap<AccountId, Account>,
    state: RunningState,
}

#[ext_contract(ext_pair)]
trait Pair {
    fn get_reserves(&self) -> (Balance, Balance);
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn add_liquidity_callback(
        &self,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        pair: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) -> String;

    fn ft_transfer_callback(&mut self) -> String;
}

pub mod gas {
    use near_sdk::Gas;
    const BASE: Gas = 10_000_000_000_000;
    pub const CALL: Gas = BASE * 2;
    pub const CALLBACK: Gas = BASE * 2;
}

const NO_DEPOSIT: Balance = 0;
const ONE_YOCTO: Balance = 1;
const MIN_STORAGE_BALANCE: Balance = 12_500_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum RunningState {
    Running,
    Paused,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            factory: String::from("factory.kula.testnet"),
            accounts: LookupMap::new(StorageKey::Accounts),
            state: RunningState::Running,
        }
    }

    #[payable]
    pub fn add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
    ) -> Promise {
        let pair_address = get_pair_name(token_a.clone(), token_b.clone(), self.factory.clone());
        println!("{}", pair_address);
        ext_pair::get_reserves(&pair_address, 0, gas::CALL).then(ext_self::add_liquidity_callback(
            amount_a_desired,
            amount_b_desired,
            pair_address,
            token_a,
            token_b,
            &env::current_account_id(),
            NO_DEPOSIT,
            gas::CALL * 5,
        ))
    }

    #[private]
    pub fn add_liquidity_callback(
        &mut self,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        pair: AccountId,
        token_a: AccountId,
        token_b: AccountId,
    ) -> String {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
            PromiseResult::Successful(val) => "success".into(),
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// 当转入此账户时， 自动记录用户的账户信息和充值金额
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        env::log(token_in.as_bytes());
        env::log("test ft_on_transfer kula".as_bytes());
        PromiseOrValue::Value(U128(0))
    }
}

/// Internal methods implementation.
impl Contract {
    /// 判断合约是否在运行状态
    fn assert_contract_running(&self) {
        match self.state {
            RunningState::Running => (),
            _ => env::panic(ERR51_CONTRACT_PAUSED.as_bytes()),
        };
    }

    pub fn internal_get_account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id)
    }

    pub fn internal_unwrap_or_default_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account(account_id)
            .unwrap_or_else(|| Account::new(account_id))
    }

    pub fn internal_register_account(&mut self, account_id: &AccountId, amount: Balance) {
        let mut account = self.internal_unwrap_or_default_account(account_id);
        account.near_amount += amount;
        self.internal_save_account(account);
    }

    pub fn internal_save_account(&mut self, account: Account) {
        account.assert_storage_usage();
        self.accounts.insert(&account.account_id.clone(), &account);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    pub fn ntoy(near_amount: Balance) -> Balance {
        near_amount * 10u128.pow(24)
    }
    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id)
            .attached_deposit(ntoy(31));
        builder
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1).into());
        testing_env!(context.build());
        let mut contract = Contract::new();
        contract.add_liquidity(
            String::from("kula.kula.testnet"),
            String::from("ayat.kula.testnet"),
            10,
            10,
        );
        // test the code
        println!("hello");
    }
}
