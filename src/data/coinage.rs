pub struct Coinage {
  gold: u32,
  silver: u32,
  copper: u32,
}

use crate::data::coin::Coin;

// Methods
impl Coinage {
  pub fn get_coinage(&self, coin: Coin) -> u32 {
    match coin {
      Coin::Gold => self.gold,
      Coin::Silver => self.silver,
      Coin::Copper => self.copper,
      _ => panic!("Invalid coinage")
    }
  }

  pub fn set_coinage(&mut self, coin: Coin, amount: u32) {
    match coin {
      Coin::Gold => self.gold = amount,
      Coin::Silver => self.silver = amount,
      Coin::Copper => self.copper = amount,
      _ => panic!("Invalid coinage")
    }
  }
}

// Constructor
impl Coinage {
  pub fn new(gold: u32, silver: u32, copper: u32) -> Coinage {
    Coinage { gold, silver, copper }
  }
}
