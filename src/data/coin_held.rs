use super::coinage::Coinage;
use super::coin::Coin;

pub struct CoinHeld {
  pub gold: Coinage,
  pub silver: Coinage,
  pub copper: Coinage,
}

impl CoinHeld {
  pub fn get_coinage(&self, coin: &Coin) -> Coinage {
    match coin {
      Coin::Gold => self.gold,
      Coin::Silver => self.silver,
      Coin::Copper => self.copper,
    }
  }

  pub fn list_coinage(&self) -> String {
    format!("{}gp, {}sp, {}cp", self.gold.amount, self.silver.amount, self.copper.amount)
  }
}

// Constructor
impl CoinHeld {
  pub fn new(gold: Coinage, silver: Coinage, copper: Coinage) -> CoinHeld {
    CoinHeld { gold, silver, copper }
  }
}
