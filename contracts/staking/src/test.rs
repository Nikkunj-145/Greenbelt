#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, String};
use token::{TokenContract, TokenContractClient};

struct Setup<'a> {
    env: Env,
    user: Address,
    pool_addr: Address,
    pool: StakingContractClient<'a>,
    token: TokenContractClient<'a>,
}

fn setup<'a>() -> Setup<'a> {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy token
    let admin = Address::generate(&env);
    let token_id = env.register(TokenContract, ());
    let token = TokenContractClient::new(&env, &token_id);
    token.init(&admin, &7u32, &String::from_str(&env, "Stake"), &String::from_str(&env, "STK"));

    // Deploy staking pool
    let pool_id = env.register(StakingContract, ());
    let pool = StakingContractClient::new(&env, &pool_id);
    pool.init(&token_id);

    // Mint tokens to user
    let user = Address::generate(&env);
    token.mint(&user, &10_000);

    Setup { env, user, pool_addr: pool_id, pool, token }
}

#[test]
fn stake_locks_tokens_into_pool() {
    let s = setup();
    s.pool.stake(&s.user, &500);
    assert_eq!(s.pool.staked(&s.user), 500);
    assert_eq!(s.token.balance(&s.user), 9_500);
    assert_eq!(s.token.balance(&s.pool_addr), 500);
    assert_eq!(s.pool.total_staked(), 500);
}

#[test]
fn unstake_returns_tokens() {
    let s = setup();
    s.pool.stake(&s.user, &800);
    s.pool.unstake(&s.user, &300);
    assert_eq!(s.pool.staked(&s.user), 500);
    assert_eq!(s.token.balance(&s.user), 9_500);
    assert_eq!(s.token.balance(&s.pool_addr), 500);
}

#[test]
fn rewards_accrue_over_time() {
    let s = setup();
    s.pool.stake(&s.user, &1_000);

    // Advance ledger time
    s.env.ledger().with_mut(|li| li.timestamp += 10_000);

    let pts = s.pool.pending_points(&s.user);
    // amount * elapsed * rate / SCALE = 1000 * 10000 * 100 / 1e9 = 1
    assert!(pts >= 1);
}

#[test]
fn double_init_rejected() {
    let s = setup();
    let res = s.pool.try_init(&s.pool_addr);
    assert!(res.is_err());
}

#[test]
fn unstake_without_stake_fails() {
    let s = setup();
    let res = s.pool.try_unstake(&s.user, &100);
    assert!(res.is_err());
}

#[test]
fn cannot_unstake_more_than_staked() {
    let s = setup();
    s.pool.stake(&s.user, &100);
    let res = s.pool.try_unstake(&s.user, &500);
    assert!(res.is_err());
}

#[test]
fn negative_amount_rejected() {
    let s = setup();
    let res = s.pool.try_stake(&s.user, &-1);
    assert!(res.is_err());
}

#[test]
fn claim_settles_pending_into_points() {
    let s = setup();
    s.pool.stake(&s.user, &10_000);
    s.env.ledger().with_mut(|li| li.timestamp += 1_000_000);
    let pts = s.pool.claim(&s.user);
    assert!(pts > 0);
}
