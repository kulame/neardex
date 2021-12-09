use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub factory: AccountId,
    pub token_a: AccountId,
    pub token_b: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(token_a: AccountId, token_b: AccountId) -> Self {
        let factory = env::signer_account_id();
        println!("{}", token_a);
        let this = Self {
            factory: factory,
            token_a: token_a,
            token_b: token_b,
        };
        this
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
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
        let _contract = Contract::new(accounts(2).into(), accounts(3).into());
    }
}
