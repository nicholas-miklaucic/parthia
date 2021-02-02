//! The data types required to work with randomness in Fire Emblem.
//!
//! The most important thing to know about randomness in Fire Emblem is that the
//! majority of the series does not display the true values underpinning the hit
//! rate or level up systems. Most games use a system that attempts to match
//! human psychology better by making unlikely events even less likely and
//! making likely events even more so, so a 90% hit rate may actually represent
//! a 99% chance to hit.
//!
//! The other thing to know is that Fire Emblem is fundamentally deterministic:
//! the same actions will result in the same outcomes. You can think of it as
//! playing Monopoly where, instead of rolling new dice for each turn, you roll
//! 1000 dice at the start of the game, and then every time you want to do
//! something with a die roll you simply read off the next values from the list.
//! You can see this with tools that let you save the game state during battles
//! or rewind time (at least the *Three Houses* version that preserves RNG): if
//! the next attack's critical number comes up as 1, then *any* attack you
//! choose to do on that turn with a critical rate of 1 or higher will crit.
//!
//! Different Fire Emblem games have different approaches to dealing with
//! randomness, and so a unified approach is difficult. This file tries to make
//! that easier.

/// One of the different RN systems used to compute hits and misses.
pub enum RNSystem {
    /// The honest approach: a 95% hit rate means a 95% chance of hitting, using
    /// a single random number for the calculation.
    OneRN,

    /// The hybrid approach used in *Fates* games: below 50%, one number is
    /// used, and above 50% one RNs is used but manipulated in a way that tries to
    /// split the difference between the 1RN and 2RN hit rates.
    FatesRN,

    /// The approach used in most Fire Emblem games: two numbers from 0-100 are
    /// used, and the average of those numbers is compared to the hit rate. This
    /// means that 90% listed hit rate corresponds to 99% hit rate (the chance
    /// two numbers 0-100 average to above 90 is much smaller than a single
    /// number being above 90).
    TwoRN,
}

impl RNSystem {
    /// Returns the true hit rate, as a number between 0 and 1, for a listed hit
    /// rate as described in the enum declaration.
    pub fn true_hit(&self, listed_hit: u32) -> f64 {
        let lh = listed_hit as f64;
        match self {
            RNSystem::OneRN => lh / 100.0,
            // there's no formula for this that's easier than just enumerating
            // the possibilities
            // if this is a performance bottleneck, just store the values,
            // there's only 101 of them
            RNSystem::TwoRN => {
                let mut num_hits = 0;
                for i in 0..100 {
                    for j in 0..100 {
                        if i + j < listed_hit * 2 {
                            num_hits += 1;
                        }
                    }
                }
                (num_hits as f64) / (100.0 * 100.0)
            }

            RNSystem::FatesRN => if listed_hit < 50 {
                lh / 100.0
            } else {
                // this is a weird formula!
                // https://www.reddit.com/r/fireemblem/comments/ae5666/echoes_absolutely_uses_fates_rn_bonus_explanation/
                (lh + ((4.0 / 30.0) * lh * ((0.02 * lh - 1.0) * 180.0).to_radians().sin())) / 100.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onern_rng() {
        assert!((RNSystem::OneRN.true_hit(70) - 0.7).abs() <= 0.01);
    }

    #[test]
    fn test_fates_rng() {
        assert!((RNSystem::FatesRN.true_hit(70) - 0.7887).abs() <= 0.01);
    }

    #[test]
    fn test_tworn_rng() {
        assert!((RNSystem::TwoRN.true_hit(70) - 0.823).abs() <= 0.01);
    }
}
