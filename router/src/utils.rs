use near_sdk::AccountId;

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
