//! This file defines an enumeration that accounts for the different ways games
//! handle basic mechanics that can't be compartmentalized into units or
//! weapons. For example, different games use different level-up systems,
//! different hit rate systems, and different critical damage calculations. The
//! hit rate systems are dealt with by the `rng` module but encapsulated here as
//! well.

use crate::rng::RNSystem;
use strum_macros::{Display, EnumString, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, EnumString, EnumIter,
         Deserialize, Serialize)]
pub enum FEGame {
    FE1,
    FE2,
    FE3,
    FE4,
    FE5,
    FE6,
    FE7,
    FE8,
    FE9,
    FE10,
    FE11,
    FE12,
    FE13,
    FE14,
    FE15,
    SoV,
}


impl FEGame {
    /// Converts listed hit, what the game tells you the hit rate is, to true
    /// hit, the actual hit probability. Most of the games lie to you about
    /// this: the full details are in the `rng` module.
    pub fn true_hit(&self, listed_hit: u32) -> f64 {
        match self {
            FEGame::FE1 | FEGame::FE2 | FEGame::FE3 | FEGame::FE4 |
             FEGame::FE5 => RNSystem::OneRN.true_hit(listed_hit),
            FEGame::FE14 | FEGame::SoV =>
                RNSystem::FatesRN.true_hit(listed_hit),
            _ => RNSystem::TwoRN.true_hit(listed_hit)
        }
    }

    /// Computes critical damage: this is done by doubling Atk in FE4 and FE5,
    /// but done by tripling damage (Atk - Def) in the other games.
    pub fn crit_damage(&self, atk: u32, def: u32) -> u32 {
        match self {
            FEGame::FE4 | FEGame::FE5 => atk * 2 - def,
            _ => (atk - def) * 3
        }
    }
}
