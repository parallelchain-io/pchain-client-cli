/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Stake to a format which can be displayed on the terminal.

use serde::Serialize;
use crate::command::Base64String;

/// [Stake] denotes a display_types equivalent of
/// pchain_types::Stake.
#[derive(Serialize, Debug)]
pub struct Stake {
    pub owner: Base64String,
    pub power: u64
}

impl From<pchain_types::Stake> for Stake {
    fn from(stake: pchain_types::Stake) -> Stake {
        Stake {
            owner: pchain_types::Base64URL::encode(&stake.owner).to_string(),
            power: stake.power
        }
    }
}

/// [Deposit] denotes a display_types equivalent of
/// pchain_types::Deposit.
#[derive(Serialize, Debug)]
pub struct Deposit {
    pub balance: u64,
    pub auto_stake_rewards: bool,
}

impl From<pchain_types::rpc::Deposit> for Deposit {
    fn from(deposit: pchain_types::rpc::Deposit) -> Deposit {
        Deposit {
            balance: deposit.balance,
            auto_stake_rewards: deposit.auto_stake_rewards
        }
    }
}

/// [Pool] denotes a display_types equivalent of
/// pchain_types::Pool.
#[derive(Serialize, Debug)]
pub enum Pool {
    WithStakes(PoolWithDelegators),
    WithoutStakes(PoolWithoutDelegators),
}

impl From<pchain_types::rpc::Pool> for Pool {
    fn from(pool: pchain_types::rpc::Pool) -> Pool {
        match pool {
            pchain_types::rpc::Pool::WithStakes(p) => Pool::WithStakes(From::<pchain_types::rpc::PoolWithDelegators>::from(p)),
            pchain_types::rpc::Pool::WithoutStakes(p) => Pool::WithoutStakes(From::<pchain_types::rpc::PoolWithoutDelegators>::from(p)),
        }
    }
}


/// [PoolWithoutDelegatedStakes] denotes a display_types equivalent of
/// pchain_types::PoolWithoutDelegatedStakes.
#[derive(Serialize, Debug)]
pub struct PoolWithoutDelegators {
    pub operator: Base64String,
    pub commission_rate: u8,
    pub power: u64,
    pub operator_stake: Option<Stake>
}

impl From<pchain_types::rpc::PoolWithoutDelegators> for PoolWithoutDelegators {
    fn from(pool: pchain_types::rpc::PoolWithoutDelegators) -> PoolWithoutDelegators {
        let operator_stake = if pool.operator_stake.is_some() {
            let pool: Stake  = From::<pchain_types::Stake>::from(pool.operator_stake.unwrap());
            Some(pool)
        } else {
            None
        };

        PoolWithoutDelegators {
            operator: pchain_types::Base64URL::encode(&pool.operator).to_string(),
            commission_rate: pool.commission_rate,
            power: pool.power,
            operator_stake
        }
    }
}

/// [PoolWithDelegators] denotes a display_types equivalent of
/// pchain_types::Pool.
#[derive(Serialize, Debug)]
pub struct PoolWithDelegators {
    pub operator: Base64String,
    pub commission_rate: u8,
    pub power: u64,
    pub operator_stake: Option<Stake>,
    pub delegated_stakes: Vec<Stake>
}

impl From<pchain_types::rpc::PoolWithDelegators> for PoolWithDelegators {
    fn from(pool: pchain_types::rpc::PoolWithDelegators) -> PoolWithDelegators {
        let operator_stake = if pool.operator_stake.is_some() {
            let pool: Stake  = From::<pchain_types::Stake>::from(pool.operator_stake.unwrap());
            Some(pool)
        } else {
            None
        };

        let delegated_stakes: Vec<Stake> = pool.delegated_stakes.into_iter().map(|stake| From::<pchain_types::Stake>::from(stake)).collect();

        PoolWithDelegators {
            operator: pchain_types::Base64URL::encode(&pool.operator).to_string(),
            commission_rate: pool.commission_rate,
            power: pool.power,
            operator_stake,
            delegated_stakes
        }
    }
}

/// [NextValidator] displays information of validator selected 
/// for the next epoch on ParallelChain.
#[derive(Serialize, Debug)]
pub struct NextValidator {
    pub address: Base64String,
    pub power: u64,
}

/// [ValidatorSet] displays information of validator set 
/// with or without the stakers' information.
#[derive(Serialize, Debug)]
pub enum ValidatorSet {
    WithDelegators(Vec<PoolWithDelegators>),
    WithoutDelegators(Vec<PoolWithoutDelegators>),
}

impl From<pchain_types::rpc::ValidatorSet> for ValidatorSet {
    fn from(vs: pchain_types::rpc::ValidatorSet) -> ValidatorSet {
        match vs {
            pchain_types::rpc::ValidatorSet::WithDelegators(pool_vec) => {
                ValidatorSet::WithDelegators(pool_vec.into_iter().map(|pool| From::<pchain_types::rpc::PoolWithDelegators>::from(pool)).collect())
            },
            pchain_types::rpc::ValidatorSet::WithoutDelegators(pool_vec) => {
                ValidatorSet::WithoutDelegators(pool_vec.into_iter().map(|pool| From::<pchain_types::rpc::PoolWithoutDelegators>::from(pool)).collect())
            },
        }
    }
}
