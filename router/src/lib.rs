use near_sdk::AccountId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, near_bindgen, Balance, PanicOnDefault, Promise, PromiseResult,
};
use utils::get_pair_name;

mod utils;
use crate::utils::{quote, sort_tokens};

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
    fn add_liquidity_callback() -> String;
}
pub mod gas {
    use near_sdk::Gas;
    const BASE: Gas = 25_000_000_000_000;
    pub const CALL: Gas = 30_000_000_000_000;
    pub const CALLBACK: Gas = BASE * 2;
}

const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            factory: String::from("factory.kula.testnet"),
        }
    }

    pub fn add_liquidity(&self, token_a: AccountId, token_b: AccountId) -> Promise {
        let pair_address = get_pair_name(token_a, token_b, self.factory.clone());
        println!("{}", pair_address);
        ext_pair::get_reserves(&pair_address, 0, gas::CALL).then(ext_self::add_liquidity_callback(
            &env::current_account_id(),
            0,
            gas::CALLBACK,
        ))
    }

    #[private]
    pub fn add_liquidity_callback(&mut self) -> String {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
            PromiseResult::Successful(val) => {
                if let Ok((reverse1, reverse2)) =
                    near_sdk::serde_json::from_slice::<(Balance, Balance)>(&val)
                {
                    format!("{},{}", reverse1, reverse2)
                } else {
                    String::from("nodata")
                }
            }
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
    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
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
        );
    }
}
