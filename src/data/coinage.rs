pub struct Coinage {
  gold: u32,
  silver: u32,
  copper: u32,
}

impl Coinage {

  pub fn new(gold: u32, silver: u32, copper: u32) -> Coinage {
    Coinage { gold, silver, copper }
  }

  pub fn get_coinage(&self) -> Vec<u32> {
    vec![self.gold, self.silver, self.copper]
  }

}