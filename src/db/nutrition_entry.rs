use std::collections::HashMap;

use super::types::NutritionEntry;

impl NutritionEntry {
  pub fn new() -> Self {
    Self {
      name: "".to_string(),
      data: HashMap::new(),
    }
  }

  pub fn vals_per_amt(&self, amount: f64) -> HashMap<String, f64> {
    if self.data["serv_size"] == amount {
      return self.data.clone();
    }

    let mut result = HashMap::new();
    let ratio = amount / self.data["serv_size"];
    for (key, val) in self.data.iter() {
      result.insert(key.clone(), val * ratio);
    }
    result
  }

  pub fn per_100g(&mut self) -> HashMap<String, f64> {
    self.vals_per_amt(100.0)
  }
}
