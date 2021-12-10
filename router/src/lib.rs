use near_sdk::AccountId;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, Balance, PanicOnDefault,
};

mod utils;
use crate::utils::sort_tokens;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub factory: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            factory: String::from("factory.kula.testnet"),
        }
    }

    pub fn add_liquidity(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: Balance,
        amount_b_desired: Balance,
        amount_a_min: Balance,
        amount_b_min: Balance,
    ) -> (Balance, Balance) {
        let (token1, token2) = sort_tokens(token_a, token_b);
        let pair = format!("{}.{}", token1, token2).replace(".", "-");
        let pair_address = format!("{}.{}", pair, self.factory);
        return (amount_a_desired, amount_b_desired);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
