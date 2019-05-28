use super::coinage::Coinage;
use super::coin::Coin;

pub struct CoinHeld {
  pub gold: Coinage,
  pub silver: Coinage,
  pub copper: Coinage,
}

impl CoinHeld {
  pub fn get_coinage(&mut self, coin: &Coin) -> &mut Coinage {
    match coin {
      Coin::Gold => &mut self.gold,
      Coin::Silver => &mut self.silver,
      Coin::Copper => &mut self.copper,
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
