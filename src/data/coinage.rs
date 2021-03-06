use super::coin::Coin;

pub struct Coinage {
  pub coin_type: Coin,
  pub amount: u32,
}

impl Coinage {
  pub fn add_amount(&mut self, update: u32) {
    self.amount += update;
  }

  pub fn sub_amount(&mut self, update: u32) {
    self.amount -= update;
  }
}

impl Coinage {
  pub fn new(coin_type: Coin, amount: u32) -> Coinage {
    Coinage { coin_type, amount }
  }
}