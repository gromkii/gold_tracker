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

  pub fn get_currency(&self) -> Vec<u16> {
    let mut v: Vec<u16> = Vec::new();
    v.push(self.gold_held);
    v.push(self.silver_held);
    v.push(self.copper_held);

    v
  }
}