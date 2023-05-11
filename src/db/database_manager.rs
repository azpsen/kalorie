use anyhow::Error;
use rusqlite::Connection;
use std::collections::HashMap;

use super::types::*;

impl DatabaseManager {
  pub fn new(db_path: String) -> Result<Self, Error> {
    println!("Opening database {db_path}...");
    let c = Connection::open(&db_path)?;
    println!("Successfully opened database\n");

    Ok(Self {
      settings: SettingsManager::new(),
      nutri_data: Vec::new(),
      journal: JournalManager::new(),
      db_path: db_path,
      conn: c,
    })
  }

  /* SETTINGS MANAGEMENT */
  pub fn load_settings(&mut self) -> Result<(), Error> {
    self.settings.load(&self.conn)
  }

  pub fn write_settings(&mut self, id: &str, val: &str) -> Result<(), Error> {
    self.settings.write(id, val, &self.conn)
  }

  pub fn get_settings(&self) -> Result<HashMap<String, SettingsEntry>, Error> {
    self.settings.get()
  }

  /* JOURNAL MANAGEMENT */
  pub fn load_journal(&mut self) -> Result<(), Error> {
    self.journal.load(&self.conn)
  }

  pub fn insert_into_journal(&mut self, entry: &FoodEntry) -> Result<(), Error> {
    self.journal.insert(entry, &self.conn)
  }

  pub fn get_journal(&self) -> Result<HashMap<u16, FoodEntry>, Error> {
    self.journal.get()
  }
}
