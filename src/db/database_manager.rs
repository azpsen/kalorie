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
      history: Vec::new(),
      db_path: db_path,
      conn: c,
    })
  }

  pub fn load_settings(&mut self) -> Result<(), Error> {
    self.settings.load(&self.conn)
  }

  pub fn write_settings(&mut self, settings: &HashMap<String, SettingsEntry>) -> Result<(), Error> {
    self.settings.write(settings, &self.conn)
  }

  pub fn get_settings(&self) -> Result<HashMap<String, SettingsEntry>, Error> {
    self.settings.get()
  }
}
