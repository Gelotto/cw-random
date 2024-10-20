use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand::prelude::*;
use cosmwasm_std::{Addr, Int128, Int256, Int64, Uint128, Uint256, Uint64};

use crate::msg::{RequestMsg, Job};
use crate::client::JobResult;


pub fn execute_job(job: Job, initial_seed: u64) -> JobResult {
    let mut rng = ChaCha8Rng::seed_from_u64(initial_seed);
    let job_results:JobResult = match job {
        Job::U8 { min, max, n } => {
            let min = min.unwrap_or(0);
            let max = max.unwrap_or(u8::MAX);
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(rng.gen_range(min..=max));
            }
            JobResult::U8(results)
        }
        Job::U16 { min, max, n } => {
            let min = min.unwrap_or(0);
            let max = max.unwrap_or(u16::MAX);
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(rng.gen_range(min..=max));
            }
            JobResult::U16(results)
        }
        Job::U32 { min, max, n } => {
            let min = min.unwrap_or(0);
            let max = max.unwrap_or(u32::MAX);
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(rng.gen_range(min..=max));
            }
            JobResult::U32(results)
        }
        Job::U64 { min, max, n } => {
            let min = u64::from(min.unwrap_or(Uint64::MIN));
            let max = u64::from(max.unwrap_or(Uint64::MAX));
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(Uint64::from(rng.gen_range(min..=max)));
            }
            JobResult::U64(results)
        }
        Job::U128 { min, max, n } => {
            let min = u128::from(min.unwrap_or(Uint128::MIN));
            let max = u128::from(max.unwrap_or(Uint128::MAX));
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(Uint128::from(rng.gen_range(min..=max)));
            }
            JobResult::U128(results)
        }
        // Job::U256 { min, max, n } => {
        //     let min = min.unwrap_or(Uint256::zero());
        //     let max = max.unwrap_or(Uint256::MAX);
        //     let mut results = Vec::with_capacity(n as usize);
        //     for _ in 0..n {
        //         results.push(rng.gen_range(min..=max));
        //     }
        //     results
        // }
        Job::I8 { min, max, n } => {
            let min = min.unwrap_or(i8::MIN);
            let max = max.unwrap_or(i8::MAX);
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(rng.gen_range(min..=max));
            }
            JobResult::I8(results)
        }
        Job::I16 { min, max, n } => {
            let min = min.unwrap_or(i16::MIN);
            let max = max.unwrap_or(i16::MAX);
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(rng.gen_range(min..=max));
            }
            JobResult::I16(results)
        }
        Job::I32 { min, max, n } => {
            let min = min.unwrap_or(i32::MIN);
            let max = max.unwrap_or(i32::MAX);
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(rng.gen_range(min..=max));
            }
            JobResult::I32(results)
        }
        Job::I64 { min, max, n } => {
            let min = min.unwrap_or(Int64::MIN).i64();
            let max = max.unwrap_or(Int64::MAX).i64();
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(Int64::from(rng.gen_range(min..=max)));
            }
            JobResult::I64(results)
        }
        Job::I128 { min, max, n } => {
            let min = min.unwrap_or(Int128::MIN).i128();
            let max = max.unwrap_or(Int128::MAX).i128();
            let mut results = Vec::with_capacity(n as usize);
            for _ in 0..n {
                results.push(Int128::from(rng.gen_range(min..=max)));
            }
            JobResult::I128(results)
        }
        // Job::I256 { min, max, n } => {
        //     let min = min.unwrap_or(Int256::MIN);
        //     let max = max.unwrap_or(Int256::MAX);
        //     let mut results = Vec::with_capacity(n as usize);
        //     for _ in 0..n {
        //         results.push(rng.gen_range(min..=max));
        //     }
        //     results
        // }
        Job::Choice { samples, weights, with_replacement, n } => {
            let mut results = Vec::with_capacity(n as usize);
                if with_replacement {
                    for _ in 0..n {
                        if let Some(sample) = samples.choose(&mut rng) {
                            results.push(sample.clone());
                        }
                    }
                } else {
                    let chosen_samples = samples.choose_multiple(&mut rng, n.into());
                    results.extend(chosen_samples.cloned());
                }
                JobResult::Choice(results)
            }
        };
        job_results
}