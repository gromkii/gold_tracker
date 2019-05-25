//! Math module.

use crate::data::coinage::Coinage;
use crate::data::coin::Coin;
use crate::input::command::Command;

pub fn add_coin(coinage: &Coinage, coin: &Coin, update: u32) -> u32 {
  coinage.get_coinage(coin) + update
}

pub fn sub_coin(coinage: &Coinage, coin: &Coin, update: u32) -> u32 {
  coinage.get_coinage(coin) - update
}

pub fn set_coin(coinage: &mut Coinage, coin: &Coin, update: u32, command: &Command) {
  if command == &Command::Add {
    coinage.set_coinage(&coin, add_coin(&coinage, coin, update));
  } else {
    coinage.set_coinage(&coin, sub_coin(&coinage, coin, update));
  }
}