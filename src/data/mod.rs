pub struct Currency {
  gold_held: u16,
  silver_held: u16,
  copper_held: u16
}

impl Currency {
  pub fn new(gold_held: u16, silver_held: u16, copper_held: u16) -> Currency {
    Currency { gold_held, silver_held, copper_held }
  }

  pub fn get_gold(&self) -> String {
    format!("{}gp", self.gold_held)
  }

  pub fn get_silver(&self) -> String {
    format!("{}sp", self.silver_held)
  }

  pub fn get_copper(&self) -> String { 
    format!("{}cp", self.copper_held)
  }

  pub fn get_currency(&self) -> Vec<String> {
    vec![
      self.get_gold(),
      self.get_silver(),
      self.get_copper()
    ]
  }
}
