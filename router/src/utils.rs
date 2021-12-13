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
    assert!(amount_a > 0, "INSUFFICIENT_AMOUNT");
    assert!(reserve_a > 0, "INSUFFICIENT_LIQUIDITY");
    assert!(reserve_b > 0, "INSUFFICIENT_LIQUIDITY");
    amount_a * reserve_b / reserve_a
}

pub fn get_pair_name(token_a: AccountId, token_b: AccountId, master: AccountId) -> AccountId {
    let (token1, token2) = sort_tokens(token_a, token_b);
    let pair = format!("pair_{}.{}", token1, token2).replace(".", "-");
    let pair_account_id = format!("{}.{}", pair, master);
    pair_account_id
}

pub fn get_amount(
    reverse1: Balance,
    reverse2: Balance,
    amount_a_desired: Balance,
    amount_b_desired: Balance,
) -> (Balance, Balance) {
    let amount1;
    let amount2;
    if reverse1 == 0 && reverse2 == 0 {
        amount1 = amount_a_desired;
        amount2 = amount_b_desired;
    } else {
        amount1 = 0;
        amount2 = 0;
    }
    (amount1, amount2)
}
