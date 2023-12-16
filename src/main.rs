/**
   We're going to run a simulation with min_stake to max_stake being randomly added in over a set duration with given roll intervals.

   The rolls will determine how many ships are built and what level they get to by the end of the duration.
*/
use rand::Rng;
pub fn main() {
    let mut rng = rand::thread_rng();
    let min_stake: u64 = 1000; //1k Bonk
    let max_stake: u64 = 100000; //100k Bonk
    let mint_cost_multiplier: u64 = 50; //Costs an extra 50 bonk for every sleigh that already exists
    let roll_intervals: u64 = 1; // will run for 100 intervals (for main game this would translate to 100*15m interval -- if we set 15 min intervals, so total duration would be 1500 min or about 25 hours)
    let max_new_sleighs_per_interval: u64 = 50; // an assumption that no more than X sleighs are staked between intervals

    println!("Running simulation...");
    let mut sleighs_pending: Vec<Sleigh> = vec![];
    let mut sleighs_built: Vec<Sleigh> = vec![];
    let mut current_max_stake: u64 = 0;

    for _i in 0..roll_intervals {
        // Add new Sleighs
        let new_num_sleighs = rng.gen_range(0..max_new_sleighs_per_interval);
        for _s in 0..new_num_sleighs {
            // generate a random stake for that sleigh
            let mut sleigh_stake = rng.gen_range(min_stake..max_stake);
            // only ever stake new sleighs at the min mint cost or higher
            if sleigh_stake < (sleighs_built.len() as u64 * mint_cost_multiplier) {
                sleigh_stake = sleighs_built.len() as u64 * mint_cost_multiplier;
            }
            if sleigh_stake > current_max_stake {
                current_max_stake = sleigh_stake;
            }

            sleighs_pending.push(Sleigh {
                mint_cost: 0, // changed when moved from pending to built
                stake: sleigh_stake,
                level: 0,
            });
        }

        // Make a roll
        let roll = rng.gen_range(0..current_max_stake + 1); // Try out differnt forumlas here

        // Check each built sliegh first to see if it gets upgraded
        sleighs_built = sleighs_built
            .iter_mut()
            .map(|sleigh: &mut Sleigh| {
                if sleigh.stake > roll {
                    sleigh.level += 1;
                }
                return sleigh.clone();
            })
            .collect();
        // Check each pending sleigh to see if it gets built
        let pending_sleighs = &sleighs_pending;
        for pending_sleigh in pending_sleighs {
            if pending_sleigh.stake > roll {
                let current_mint_cost = sleighs_built.len() as u64 * mint_cost_multiplier;
                if pending_sleigh.stake > current_mint_cost {
                    sleighs_built.push(Sleigh {
                        mint_cost: current_mint_cost,
                        stake: pending_sleigh.stake,
                        level: 1,
                    })
                } else {
                    println!("Mint cost greater than sleigh stake!")
                }
            }
        }
    }

    for (idx, sleigh) in sleighs_built.iter().enumerate() {
        println!(
            "Sleigh {:#}: Mint Cost {:#} | Stake {:#} | Level {:#}",
            idx, sleigh.mint_cost, sleigh.stake, sleigh.level
        );
    }
}

#[derive(Copy, Debug, Clone)]
struct Sleigh {
    mint_cost: u64,
    stake: u64,
    level: u64,
}
