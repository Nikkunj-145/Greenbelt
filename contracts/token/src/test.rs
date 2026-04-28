#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup<'a>() -> (Env, Address, TokenContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &id);
    client.init(
        &admin,
        &7u32,
        &String::from_str(&env, "Stellar Stake"),
        &String::from_str(&env, "STK"),
    );
    (env, admin, client)
}

#[test]
fn init_sets_metadata() {
    let (env, _, client) = setup();
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.symbol(), String::from_str(&env, "STK"));
}

#[test]
fn mint_and_balance() {
    let (env, _, client) = setup();
    let alice = Address::generate(&env);
    client.mint(&alice, &1_000_000);
    assert_eq!(client.balance(&alice), 1_000_000);
}

#[test]
fn transfer_moves_balance() {
    let (env, _, client) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    client.mint(&alice, &500);
    client.transfer(&alice, &bob, &200);
    assert_eq!(client.balance(&alice), 300);
    assert_eq!(client.balance(&bob), 200);
}

#[test]
fn transfer_insufficient_fails() {
    let (env, _, client) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let res = client.try_transfer(&alice, &bob, &100);
    assert!(res.is_err());
}

#[test]
fn negative_amount_rejected() {
    let (env, _, client) = setup();
    let alice = Address::generate(&env);
    let res = client.try_mint(&alice, &-100);
    assert!(res.is_err());
}

#[test]
fn faucet_grants_tokens_once() {
    let (env, _, client) = setup();
    let alice = Address::generate(&env);
    assert_eq!(client.faucet_claimed(&alice), false);
    let granted = client.faucet(&alice);
    assert_eq!(granted, 10_000_000_000);
    assert_eq!(client.balance(&alice), 10_000_000_000);
    assert_eq!(client.faucet_claimed(&alice), true);
}

#[test]
fn faucet_double_claim_rejected() {
    let (env, _, client) = setup();
    let alice = Address::generate(&env);
    client.faucet(&alice);
    let res = client.try_faucet(&alice);
    assert!(res.is_err());
}
