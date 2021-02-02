//! Represents the statistical profile of a single round of combat in Fire
//! Emblem: the hit, crit, doubling, etc. Think of this as the information in
//! the combat preview.
//!
//! This doesn't attempt to capture the full complexity of all combat: skill
//! activations, status afflictions, etc. Instead, the goal is to approximate
//! what the games themselves show in a preview: purely hit, crit, and damage,
//! as opposed to any other effects. A different system is used for this full
//! complexity, but the extra boilerplate makes it unwieldy for simple
//! calculations.


use crate::rng::RNSystem;

/// Describes how many times the attacker/defender will strike. There are two
/// kinds of doubling: repeated attacks, that occur when the striker outspeeds
/// their target by a game-dependent amount, and continued attacks, which are
/// immediate second attacks that occur from things like weapon effects, combat
/// arts, and skill activations. These can stack, so a unit can attack 4 times
/// if they outspeed their target and are using a brave weapon, for example. For
/// ease of reference, continued attacks are called "brave" after the name of
/// the weapon type that most commonly produces them.
pub struct AttackRepeat {
    /// Whether the attacker naturally outspeeds the defender.
    outspeeds: bool,
    /// Whether the attacker gets continued attacks from a brave weapon or brave
    /// combat art.
    is_brave: bool
}

/// The statistics for a single strike in a round of combat.
pub struct Attack {
    /// The listed hit rate as a percentage between 0 and 100.
    hit: u32,

    /// The critical rate as a percentage between 0 and 100.
    crit: u32,

    /// The damage dealt by the attack.
    dmg: u32,
}

impl Attack {
    /// Returns the probability (as a number 0-1) that an attack with these
    /// stats will kill a target with the given HP and RNG system.
    pub fn prob_kills(&self, rn: RNSystem, def_hp: u32) -> f64 {
        let prob_hit = rn.true_hit(self.hit);
        let prob_miss = 1.0 - prob_hit;
        let prob_crit = prob_hit * (self.crit as f64 / 100.0);
        let prob_normal_hit = prob_hit - prob_crit;
    }
}

pub struct Round {
    /// The attacker HP before combat starts.
    atk_hp: u32,

    /// The hit, crit, and damage of the attacker's action.
    attacker: Attack,

    /// The defender HP before combat starts.
    def_hp: u32,

    /// The hit, crit, and damage of the defender's action.
    defender: Attack,

    /// Describes how the attacker will strike, with possible multiple strikes.
    atk_repeat: AttackRepeat,

    /// Describes how the attacker will strike, with possible multiple strikes.
    def_repeat: AttackRepeat
}

impl Round {
    /// The probability that the attacker survives after combat concludes using
    /// the given randomness system, as a number between 0 and 1.
    pub fn prob_atk_survival(&self, rn: RNSystem) -> f64 {
        // attacker gets first strike: determine probability that this strike
        // kills
        let kills_in_one =
    }
}
