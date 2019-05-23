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
}

// Constructor
impl Coinage {
  pub fn new(gold: u32, silver: u32, copper: u32) -> Coinage {
    Coinage { gold, silver, copper }
  }
}