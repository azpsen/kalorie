use anyhow::Error;
use std::{io, thread, time::Duration};

use crate::db::types::{DatabaseManager, FoodEntry};

pub mod db;
mod ui;

fn main() -> Result<(), Error> {
  // Load database
  let mut db_man = DatabaseManager::new("kalorie.db".to_string())?;

  db_man.load_settings()?;
  let settings = db_man.settings.get()?;
  println!("Loaded settings:\n{:#?}", settings);

  db_man.load_journal()?;
  let entry = FoodEntry {
    name: "Acovado".to_string(),
    datetime: chrono::Utc::now(),
    nutrition_id: 420,
  };
  db_man.insert_into_journal(&entry)?;
  let journal = db_man.get_journal()?;
  for (k, v) in journal {
    println!("{k}: {:#?}", v);
  }

  Ok(())
}
