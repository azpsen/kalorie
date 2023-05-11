use anyhow::Error;
use rusqlite::Connection;
use std::collections::HashMap;

use super::types::*;

const DEFAULT_SETTINGS: [[&'static str; 2]; 4] = [
  ["first_open", "1"],
  ["Name", ""],
  ["Target Calories", "2000"],
  ["Target Carbohydrates", "20"],
];

impl SettingsManager {
  pub fn new() -> Self {
    Self {
      settings: HashMap::new(),
    }
  }

  pub fn load(&mut self, conn: &Connection) -> Result<(), Error> {
    // Check if 'settings' table exists
    let mut settings_table =
      conn.prepare("select * from sqlite_master where type='table' and name='settings'")?;

    if !settings_table.exists(())? {
      // Create and populate settings table in db
      println!("Settings table does not exist, creating...");
      conn.execute(
        "create table settings (
          name text primary key,
          value text not null
        )",
        (),
      )?;

      println!("Created table settings");
      println!("Inserting default settings...");

      for i in DEFAULT_SETTINGS.iter() {
        // Insert default settings into db
        conn.execute(
          "insert into settings (
            name,
            value
          )
          values (
            ?1, ?2
          )",
          (i[0], i[1]),
        )?;

        // Load default settings into memory
        self.settings.insert(i[0].to_string(), i[1].to_string());
      }
      println!("Inserted default settings\n");
    } else {
      // Load existing settings values from db
      println!("Settings table exists, loading...");
      let stmt = conn.prepare("select * from settings");
      let mut statement = stmt?;
      let mut settings_vals = statement.query([])?;

      // Add loaded values into local memory
      while let Some(v) = settings_vals.next()? {
        self.settings.insert(v.get(0)?, v.get(1)?);
      }
      println!("Loaded settings into memory\n");
    }

    Ok(())
  }

  pub fn get(&self) -> Result<HashMap<String, String>, Error> {
    Ok(self.settings.clone())
  }

  pub fn write(
    &mut self,
    settings: &HashMap<String, String>,
    conn: &Connection,
  ) -> Result<(), Error> {
    for (k, v) in settings {
      let mut stmt = conn.prepare("update settings set value = ?1 where name = ?2")?;
      stmt.execute((v, k))?;

      self.settings.insert(k.to_string(), v.to_string());
    }

    Ok(())
  }
}
