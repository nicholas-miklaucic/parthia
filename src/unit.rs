//! Represents a unit in Fire Emblem with the associated stats such units have.
//! Because different Fire Emblem games have different stats and they mean
//! different things, this is a trait that has to be implemented specifically
//! for different games.

use crate::weapon::{Weapon, Item};

/// A unit in Fire Emblem that can attack and defend.
pub trait Unit: Sized {
    /// Attacks the target with the given weapon, updating both this unit and
    /// the target.
    fn attack(&mut self, enemy: &mut Self,
              atk_weapon: dyn Weapon<Self>, def_weapon: dyn Weapon<Self>,
              atk_item: dyn Item<Self>, def_item: dyn Item<Self>);
}
