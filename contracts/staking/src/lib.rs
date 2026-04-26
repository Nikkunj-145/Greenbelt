#![no_std]
//! Simple staking pool that locks a SEP-41 fungible token and accrues
//! reward "points" linearly over time. Demonstrates **inter-contract
//! calls** by invoking the token contract for `transfer` and `balance`.
//!
//! Reward formula: points = staked_amount * seconds_elapsed * RATE / 1e9
//! (RATE = 100, i.e. 100 points per token per second, scaled).
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, contracttype, symbol_short, Address,
    Env, Symbol,
};

const REWARD_RATE: i128 = 100; // points per token per second (scaled by 1e9)
const SCALE: i128 = 1_000_000_000;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NothingStaked = 3,
    InsufficientStake = 4,
    NegativeAmount = 5,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Token,
    Stake(Address),
    Points(Address),
    TotalStaked,
}

#[contracttype]
#[derive(Clone)]
pub struct StakeInfo {
    pub amount: i128,
    pub since: u64, // ledger timestamp when last updated
}

const STAKE_EVT: Symbol = symbol_short!("stake");
const UNSTAKE_EVT: Symbol = symbol_short!("unstake");
const CLAIM_EVT: Symbol = symbol_short!("claim");

/// Minimal client for a SEP-41-compatible token (just what we need).
#[contractclient(name = "TokenClient")]
pub trait TokenInterface {
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn balance(env: Env, id: Address) -> i128;
}

#[contract]
pub struct StakingContract;

#[contractimpl]
impl StakingContract {
    pub fn init(env: Env, token: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Token) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::TotalStaked, &0i128);
        Ok(())
    }

    pub fn token(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)
    }

    /// Stake `amount` tokens from `user` into the pool.
    /// **Inter-contract call**: invokes `token.transfer(user, pool, amount)`.
    pub fn stake(env: Env, user: Address, amount: i128) -> Result<i128, Error> {
        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }
        user.require_auth();
        let token: Address = Self::token(env.clone())?;

        // Settle existing rewards before mutating stake
        Self::settle(&env, &user);

        // Inter-contract call — pull tokens from user into this contract
        let pool = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&user, &pool, &amount);

        let key = DataKey::Stake(user.clone());
        let mut info: StakeInfo = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(StakeInfo { amount: 0, since: env.ledger().timestamp() });
        info.amount += amount;
        info.since = env.ledger().timestamp();
        env.storage().persistent().set(&key, &info);

        let total: i128 = env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalStaked, &(total + amount));

        env.events().publish((STAKE_EVT, user.clone()), (amount, info.amount));
        Ok(info.amount)
    }

    /// Unstake `amount` tokens. Settles rewards first.
    pub fn unstake(env: Env, user: Address, amount: i128) -> Result<i128, Error> {
        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }
        user.require_auth();
        let token: Address = Self::token(env.clone())?;
        let key = DataKey::Stake(user.clone());
        let mut info: StakeInfo = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NothingStaked)?;
        if info.amount < amount {
            return Err(Error::InsufficientStake);
        }

        Self::settle(&env, &user);

        // Inter-contract call — push tokens from pool back to user
        let pool = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&pool, &user, &amount);

        info.amount -= amount;
        info.since = env.ledger().timestamp();
        env.storage().persistent().set(&key, &info);

        let total: i128 = env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalStaked, &(total - amount));

        env.events().publish((UNSTAKE_EVT, user.clone()), (amount, info.amount));
        Ok(info.amount)
    }

    /// Settle accrued rewards into the points balance, reset timer.
    fn settle(env: &Env, user: &Address) {
        let key = DataKey::Stake(user.clone());
        let info: Option<StakeInfo> = env.storage().persistent().get(&key);
        if let Some(mut info) = info {
            if info.amount > 0 {
                let now = env.ledger().timestamp();
                let elapsed = now.saturating_sub(info.since) as i128;
                let earned = info.amount * elapsed * REWARD_RATE / SCALE;
                let pkey = DataKey::Points(user.clone());
                let cur: i128 = env.storage().persistent().get(&pkey).unwrap_or(0);
                env.storage().persistent().set(&pkey, &(cur + earned));
                info.since = now;
                env.storage().persistent().set(&key, &info);
            }
        }
    }

    /// Claim pending rewards as points. Returns total points after claim.
    pub fn claim(env: Env, user: Address) -> Result<i128, Error> {
        user.require_auth();
        Self::settle(&env, &user);
        let pts: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Points(user.clone()))
            .unwrap_or(0);
        env.events().publish((CLAIM_EVT, user), pts);
        Ok(pts)
    }

    /// View: current stake amount.
    pub fn staked(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get::<DataKey, StakeInfo>(&DataKey::Stake(user))
            .map(|i| i.amount)
            .unwrap_or(0)
    }

    /// View: pending (unsettled) + already-settled points.
    pub fn pending_points(env: Env, user: Address) -> i128 {
        let info: Option<StakeInfo> = env.storage().persistent().get(&DataKey::Stake(user.clone()));
        let already: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Points(user))
            .unwrap_or(0);
        let pending = match info {
            Some(i) if i.amount > 0 => {
                let elapsed = env.ledger().timestamp().saturating_sub(i.since) as i128;
                i.amount * elapsed * REWARD_RATE / SCALE
            }
            _ => 0,
        };
        already + pending
    }

    pub fn total_staked(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
