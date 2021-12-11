use near_sdk::{AccountId, Balance};

pub fn sort_tokens(token_a: AccountId, token_b: AccountId) -> (AccountId, AccountId) {
    let token1: AccountId;
    let token2: AccountId;
    if token_a > token_b {
        token1 = token_b;
        token2 = token_a;
    } else {
        token1 = token_a;
        token2 = token_b;
    }
    (token1, token2)
}

pub fn quote(amount_a: Balance, reserve_a: Balance, reserve_b: Balance) -> Balance {
    amount_a * reserve_b / reserve_a
}
