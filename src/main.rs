use anyhow::Error;
use std::{io, thread, time::Duration};

use crate::db::types::DatabaseManager;

pub mod db;
mod ui;

fn main() -> Result<(), Error> {
  // Load database
  let mut db_man = DatabaseManager::new("kalorie.db".to_string())?;

  db_man.load_settings()?;
  let mut settings = db_man.settings.get()?;
  println!("Loaded settings:\n{:#?}", settings);
  println!("\nChanging settings...\n");

  settings.get_mut("name").unwrap().value = "Hello, World!".to_string();
  db_man.write_settings(&settings)?;

  println!("Changed settings to:\n{:#?}\n", settings);
  settings = db_man.get_settings()?;
  println!("Loaded settings:\n{:#?}\n", settings);

  Ok(())
}
