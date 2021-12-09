use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
};
use near_sdk::{near_bindgen, AccountId};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub fee_to: AccountId,
    pub get_pair: LookupMap<AccountId, LookupMap<AccountId, AccountId>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            fee_to: String::from("kula.testnet"),
            get_pair: LookupMap::new(b"factory".to_vec()),
        }
    }

    fn create_pair(&self, token_a: AccountId, token_b: AccountId) -> AccountId {
        let token1: AccountId;
        let token2: AccountId;
        if token_a > token_b {
            token1 = token_b;
            token2 = token_a;
        } else {
            token1 = token_a;
            token2 = token_b;
        }
        return token1;
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
        let _contract = Contract::new();
        _contract.create_pair(
            String::from("kula.kula.testnet"),
            String::from("ayat.kula.testnet"),
        );
    }
}
