pub struct Coinage {
  gold: u32,
  silver: u32,
  copper: u32,
}

// Methods
impl Coinage {

  pub fn get_coinage(&self) -> Vec<u32> {
    vec![self.gold, self.silver, self.copper]
  }

  pub fn get_gold(&self) -> u32 {
    self.gold
  }

  pub fn set_gold(&mut self, update_gold: u32) {
    self.gold = update_gold;
  }
}

// Constructor
impl Coinage {
  pub fn new(gold: u32, silver: u32, copper: u32) -> Coinage {
    Coinage { gold, silver, copper }
  }
}