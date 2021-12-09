use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSchema, BorshSerialize},
    collections::{LookupMap, Vector},
    BorshStorageKey,
};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Balance, Promise};
use near_sdk::{
    serde::{Deserialize, Serialize},
    PromiseOrValue,
};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub fee_to: AccountId,
    pub get_pair: LookupMap<AccountId, LookupMap<AccountId, AccountId>>,
    pub all_paris: Vector<AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    GetPair,
    AllParis,
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn on_pair_created(&mut self, pair_account_id: AccountId) -> Promise;
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PairArgs {
    token_a: AccountId,
    token_b: AccountId,
}
const NO_DEPOSIT: Balance = 0;

pub mod gas {
    use near_sdk::Gas;

    /// The base amount of gas for a regular execution.
    const BASE: Gas = 25_000_000_000_000;
    pub const PAIR_NEW: Gas = BASE * 2;
    pub const CALLBACK: Gas = BASE * 2;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            fee_to: String::from("kula.testnet"),
            get_pair: LookupMap::new(StorageKeys::GetPair),
            all_paris: Vector::new(StorageKeys::AllParis),
        }
    }

    fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> Promise {
        let token1: AccountId;
        let token2: AccountId;
        if token_a > token_b {
            token1 = token_b;
            token2 = token_a;
        } else {
            token1 = token_a;
            token2 = token_b;
        }
        let pair_account_id = format!("{}.{}.{}", token1, token2, env::current_account_id());
        assert!(
            env::is_valid_account_id(pair_account_id.as_bytes()),
            "The staking pool account ID is invalid"
        );
        println!("{}", pair_account_id);
        Promise::new(pair_account_id.clone())
            .create_account()
            .transfer(env::attached_deposit())
            .deploy_contract(include_bytes!("../../res/pair.wasm").to_vec())
            .function_call(
                b"new".to_vec(),
                near_sdk::serde_json::to_vec(&PairArgs {
                    token_a: token1,
                    token_b: token2,
                })
                .unwrap(),
                NO_DEPOSIT,
                gas::PAIR_NEW,
            )
            .then(ext_self::on_pair_created(
                pair_account_id,
                &env::current_account_id(),
                NO_DEPOSIT,
                gas::CALLBACK,
            ))
    }

    pub fn on_pair_created(&mut self, pair_account_id: AccountId) -> PromiseOrValue<bool> {
        println!("{}", pair_account_id);
        PromiseOrValue::Value(true)
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
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut _contract = Contract::new();
        _contract.create_pair(
            String::from("kula.kula.testnet"),
            String::from("ayat.kula.testnet"),
        );
    }
}
