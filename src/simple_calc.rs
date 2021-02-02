//! This is a simple calculator that avoids the full complexity of the FE games
//! (lifesteal, abilities, held items, personal weapons, etc.) to focus on the
//! stats as they appear in all FE games, providing basic survival
//! probabilities.

use crate::fegame::FEGame;

use serde::{Deserialize, Serialize};


/// The stats needed for one side of combat: damage, hit, brave effect, and
/// crit.
#[derive(Default, Debug, Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CombatStats {
    /// The damage dealt.
    pub dmg: u32,

    /// The hit probability (0-100).
    pub hit: u32,

    /// The critical probability (0-100).
    pub crit: u32,

    /// Whether the weapon strikes twice per normal strike. Although usually
    /// called brave weapons, other weapons like gauntlets or the Amiti do this
    /// as well.
    pub is_brave: bool
}

impl CombatStats {
    /// Computes possible outcomes for a single round of combat using the given
    /// statistics. Doesn't deal with FE4 or FE5 crit damage correctly.
    pub fn possible_outcomes(&self, game: FEGame, outcomes: Vec<Outcome>) -> Vec<Outcome> {
        let after_one = self.after_single_strike(game, outcomes);
        if self.is_brave {
            // strike again
            self.after_single_strike(game, after_one)
        } else {
            after_one
        }
    }

    /// Returns the possible states after a single strike given the previous
    /// possible states. Critical damage is not handled correctly in FE4 and
    /// FE5.
    fn after_single_strike(&self, game: FEGame, states: Vec<Outcome>) -> Vec<Outcome> {
        let mut new_states = vec!();
        for state in states {
            if state.atk_hp == 0 {
                // dead attackers can't do anything
                new_states.push(state);
            } else {
                // three possibilities: miss, non-crit hit, and crit
                let prob_hit = game.true_hit(self.hit);
                let prob_miss = 1.0 - prob_hit;
                let prob_crit = prob_hit * self.crit as f64 / 100.0;
                let prob_reg_hit = prob_hit - prob_crit;

                // if miss, nothing happens
                new_states.push(Outcome{
                    prob: state.prob * prob_miss,
                    atk_hp: state.atk_hp,
                    def_hp: state.def_hp
                });

                // if hit, normal damage: subtract damage, cannot go negative
                new_states.push(Outcome{
                    prob: state.prob * prob_reg_hit,
                    atk_hp: state.atk_hp,
                    def_hp: state.def_hp.saturating_sub(self.dmg)
                });

                // if crit, critical damage: FE4 and FE5 critical damage
                // requires knowing Def, which we don't have, so we just do
                // triple damage like normal
                new_states.push(Outcome{
                    prob: state.prob * prob_crit,
                    atk_hp: state.atk_hp,
                    def_hp: state.def_hp.saturating_sub(3 * self.dmg)
                });
            }
        }
        Outcome::collect(new_states)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
/// The results of different speed differentials between attacker (A) and
/// defender (B), resulting in different attack patterns.
pub enum SpeedDiff {
    /// No one doubles: AB
    Even,
    /// Attacker doubles: ABA
    AtkDoubles,
    /// Defender doubles: ABB
    DefDoubles,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
/// The outcome of combat, with associated probability.
pub struct Outcome {
    pub prob: f64,
    pub atk_hp: u32,
    pub def_hp: u32,
}

impl Outcome {
    /// Combines the probabilities of identical outcomes in the list of outcomes
    /// and removes impossible outcomes, returning a new list with the same
    /// total probabilities.
    pub fn collect(outcomes: Vec<Outcome>) -> Vec<Outcome> {
        outcomes.into_iter().filter(|x| x.prob != 0.0).fold(vec![], |acc, outcome| outcome.add_into(acc))
    }

    /// Adds the outcome to the list of outcomes, adding it to the probabliity
    /// of an existing outcome if it's identical.
    pub fn add_into(&self, outcomes: Vec<Outcome>) -> Vec<Outcome> {
        let mut new_outcomes = vec!();
        let mut has_added = false;
        for outcome in outcomes {
            if (self.atk_hp == outcome.atk_hp) && (self.def_hp == outcome.def_hp) {
                new_outcomes.push(Outcome{
                    prob: self.prob + outcome.prob,
                    atk_hp: self.atk_hp,
                    def_hp: self.def_hp,
                });
                has_added = true;
            } else {
                new_outcomes.push(outcome.clone());
            }
        }
        if !has_added {
            new_outcomes.push(self.clone());
        }
        new_outcomes
    }

    /// Switches attacker and defender.
    pub fn switch(&self) -> Outcome {
        Outcome{
            prob: self.prob,
            atk_hp: self.def_hp,
            def_hp: self.atk_hp,
        }
    }
}


/// Returns a list of all of the possible outcomes of combat with associated
/// probability, using the given game's rules.
pub fn possible_outcomes(game: FEGame, atk: CombatStats, atk_hp: u32,
                         def: CombatStats, def_hp: u32,
                         speed: SpeedDiff) -> Vec<Outcome> {
    let initial = vec!(Outcome{
        prob: 1.0,
        atk_hp,
        def_hp,
    });

    let after_atk = atk.possible_outcomes(game, initial);
    let after_def = def.possible_outcomes(
        game,
        after_atk.into_iter().map(|x| x.switch()).collect()
    ).into_iter().map(|x| x.switch()).collect();

    match speed {
        SpeedDiff::Even => {
            // AB attack pattern
            after_def
        },
        SpeedDiff::AtkDoubles => {
            // ABA attack pattern
            atk.possible_outcomes(game, after_def)
        },
        SpeedDiff::DefDoubles => {
            // ABB attack pattern
            def.possible_outcomes(
                game,
                after_def.into_iter().map(|x| x.switch()).collect()
            ).into_iter().map(|x| x.switch()).collect()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outcomes() {
        dbg!(Outcome{prob: 1.0, atk_hp: 20, def_hp: 30}.add_into(vec!()));
        dbg!(CombatStats{
            dmg: 10, hit: 90, crit: 0, is_brave: false,
        }.possible_outcomes(FEGame::FE15,
                            vec![Outcome{prob: 1.0, atk_hp: 1, def_hp: 40}]));
        dbg!(possible_outcomes(FEGame::FE15, CombatStats{
            dmg: 10, hit: 50, crit: 0, is_brave: false,
        }, 30, CombatStats{
            dmg: 10, hit: 100, crit: 0, is_brave: false
        }, 20, SpeedDiff::AtkDoubles));
    }
}
