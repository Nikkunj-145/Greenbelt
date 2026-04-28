#![no_std]
//! A minimal SEP-41-style fungible token used by the staking pool.
//! Implements: balance, transfer, mint (admin), decimals/name/symbol.
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String,
    Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotAdmin = 3,
    InsufficientBalance = 4,
    NegativeAmount = 5,
    AlreadyClaimed = 6,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Decimals,
    Name,
    Symbol,
    Balance(Address),
    Faucet(Address),
}

const TRANSFER: Symbol = symbol_short!("transfer");
const MINT: Symbol = symbol_short!("mint");
const FAUCET: Symbol = symbol_short!("faucet");
const FAUCET_AMOUNT: i128 = 10_000_000_000; // 1,000 STK (with 7 decimals)

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn init(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Decimals, &decimal);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        Ok(())
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        let key = DataKey::Balance(to.clone());
        let cur: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(cur + amount));
        env.events().publish((MINT, admin, to), amount);
        Ok(())
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }
        from.require_auth();
        let from_key = DataKey::Balance(from.clone());
        let from_bal: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
        if from_bal < amount {
            return Err(Error::InsufficientBalance);
        }
        env.storage().persistent().set(&from_key, &(from_bal - amount));
        let to_key = DataKey::Balance(to.clone());
        let to_bal: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
        env.storage().persistent().set(&to_key, &(to_bal + amount));
        env.events().publish((TRANSFER, from, to), amount);
        Ok(())
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    /// Open faucet: anyone can claim FAUCET_AMOUNT once per address.
    pub fn faucet(env: Env, to: Address) -> Result<i128, Error> {
        to.require_auth();
        let claim_key = DataKey::Faucet(to.clone());
        if env.storage().persistent().has(&claim_key) {
            return Err(Error::AlreadyClaimed);
        }
        env.storage().persistent().set(&claim_key, &true);
        let bal_key = DataKey::Balance(to.clone());
        let cur: i128 = env.storage().persistent().get(&bal_key).unwrap_or(0);
        let new_bal = cur + FAUCET_AMOUNT;
        env.storage().persistent().set(&bal_key, &new_bal);
        env.events().publish((FAUCET, to), FAUCET_AMOUNT);
        Ok(FAUCET_AMOUNT)
    }

    /// Returns true if `addr` has already claimed from the faucet.
    pub fn faucet_claimed(env: Env, addr: Address) -> bool {
        env.storage().persistent().has(&DataKey::Faucet(addr))
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap_or(7)
    }

    pub fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Name)
            .unwrap_or(String::from_str(&env, ""))
    }

    pub fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Symbol)
            .unwrap_or(String::from_str(&env, ""))
    }
}

#[cfg(test)]
mod test;
