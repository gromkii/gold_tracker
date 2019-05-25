//! Math module.

use crate::data::coinage::Coinage;
use crate::data::coin::Coin;
use crate::input::command::Command;

pub fn add_coin(coinage: &Coinage, coin: &Coin, update: u32) -> u32 {
  coinage.get_coinage(coin) + update
}

pub fn sub_coin(coinage: &Coinage, coin: &Coin, update: u32) -> u32 {
  let amount = coinage.get_coinage(coin) - update;

  if amount > 1 {
    amount
  } else {
    panic!("Invalid amount.");
  }
}

pub fn set_coin(coinage: &mut Coinage, coin: &Coin, update: u32) {
  let c: &Coin = &coin;
  coinage.set_coinage(&coin, add_coin(&coinage, c, update));
}