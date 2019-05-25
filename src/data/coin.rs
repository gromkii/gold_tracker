pub enum Coin {
  Gold,
  Silver,
  Copper,
}

impl Coin {
  pub fn as_string(&self) -> &str {
    match self {
      &Coin::Gold => "gp",
      &Coin::Silver => "sp",
      &Coin::Copper => "cp",
    }
  }
}