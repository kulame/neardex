use near_sdk::AccountId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, near_bindgen, Balance, PanicOnDefault, Promise, PromiseResult,
};
use utils::{get_amount, get_pair_name};

mod utils;
use crate::utils::quote;
use near_sdk::json_types::ValidAccountId;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub factory: AccountId,
}

#[ext_contract(ext_pair)]
trait Pair {
    fn get_reserves(&self) -> (Balance, Balance);
}

#[ext_contract(ext_self)]
pub trait SelfSelf {
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

#[ext_contract(ext_token)]
pub trait Token {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: String, memo: Option<String>);
    fn storage_deposit(&mut self, account_id: AccountId, registration_only: Option<bool>);
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

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            factory: String::from("factory.kula.testnet"),
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
    ) -> Promise {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
            PromiseResult::Successful(val) => {
                let (reverse_a, reverse_b) =
                    near_sdk::serde_json::from_slice::<(Balance, Balance)>(&val).unwrap();
                let (amount1, amount2) =
                    get_amount(reverse_a, reverse_b, amount_a_desired, amount_b_desired);
                ext_token::storage_deposit(
                    pair.clone(),
                    Some(true),
                    &token_a,
                    MIN_STORAGE_BALANCE,
                    gas::CALL,
                )
                .then(ext_token::ft_transfer(
                    pair.clone(),
                    amount1.to_string(),
                    Some(String::from("hello")),
                    &token_a,
                    ONE_YOCTO,
                    gas::CALL,
                ))
                .then(ext_self::ft_transfer_callback(
                    &env::current_account_id(),
                    NO_DEPOSIT,
                    gas::CALL,
                ))
            }
        }
    }
    #[private]
    pub fn ft_transfer_callback(&mut self) -> String {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
            PromiseResult::Successful(val) => String::from("success"),
        }
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
    }
}
