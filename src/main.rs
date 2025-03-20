use std::collections::HashMap;

use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();
    let pool = roll_pool(&mut rng, 3, 7);
    let results = eval_pool(pool);
    println!("{:?}", results);
}

#[derive(Debug, PartialEq)]
enum DiceStatus {
    CriticalFailure,
    Failure,
    Success,
    CriticalSuccess,
}

#[derive(Debug)]
enum FinalResult {
    Botch,
    Failure,
    Success,
}

fn roll_d10(rng: &mut impl Rng) -> u32 {
    rng.random_range(1..=10)
}

fn roll_pool(rng: &mut impl Rng, pool_size: usize, success_threshold: u32) -> HashMap<u32, DiceStatus> {
    let mut pool: HashMap<u32, DiceStatus> = HashMap::new();
    for _ in 0..pool_size {
        let dice = roll_d10(rng);
        let dice_status = match dice {
            1 => DiceStatus::CriticalFailure,
            x if x < success_threshold => DiceStatus::Failure,
            x if x >= success_threshold && x < 10 => DiceStatus::Success,
            10 => DiceStatus::CriticalSuccess,
            _ => panic!("Unexpected Dice Result!"),
        };
        println!("{}: {:?}", dice, dice_status);
        pool.insert(dice, dice_status);
    }

    pool
}

fn eval_pool(pool: HashMap<u32, DiceStatus>) -> FinalResult {
    let results = pool.values();

    let mut final_result = FinalResult::Failure;
    for r in results {
        if r == &DiceStatus::Success || r == &DiceStatus::CriticalSuccess {
            final_result = FinalResult::Success;
            break;
        } else if r == &DiceStatus::CriticalFailure {
            final_result = FinalResult::Botch;
        }
    }

    final_result
}