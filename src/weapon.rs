//! This file defines a unified way of dealing with different weapons and held
//! items.

use crate::unit::Unit;

/// A weapon that is used to attack among units of type U.
pub trait Weapon<U> where U: Unit {

}

/// A held item that affects combat among units of type U.
pub trait Item<U> where U: Unit {

}
