use anyhow::Error;
use rusqlite::Connection;

use super::types::*;

impl DatabaseManager {
  pub fn new(db_path: String) -> Self {
    let c = Connection::open(&db_path).unwrap();

    Self {
      settings: SettingsManager::new(),
      nutri_data: Vec::new(),
      history: Vec::new(),
      db_path: db_path,
      conn: c,
    }
  }

  pub fn load_settings(&mut self) -> Result<(), Error> {
    self.settings.load(&self.conn)
  }

  pub fn write_settings(&mut self, settings: &UserSettings) -> Result<(), Error> {
    self.settings.write(settings, &self.conn)
  }

  pub fn get_settings(&self) -> Result<UserSettings, Error> {
    self.settings.get()
  }
}
