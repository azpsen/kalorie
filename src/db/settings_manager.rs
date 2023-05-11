use anyhow::Error;
use rusqlite::Connection;
use std::collections::HashMap;

use super::types::*;

const DEFAULT_SETTINGS: [(&'static str, &'static str, &'static str, bool); 4] = [
  ("first_open", "", "1", false),
  ("name", "Name", "", true),
  ("target_cals", "Target Calories", "2000", true),
  ("target_carbs", "Target Carbohydrates", "20", true),
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
          id text primary key,
          name varchar(50),
          value varchar(50),
          visible bool
        )",
        (),
      )?;

      println!("Created table settings");
      println!("Inserting default settings...");

      for i in DEFAULT_SETTINGS.iter() {
        // Insert default settings into db
        conn.execute(
          "insert into settings (
            id,
            name,
            value,
            visible
          )
          values (
            ?1, ?2, ?3, ?4
          )",
          (i.0, i.1, i.2, i.3),
        )?;

        // Load default settings into memory
        self.settings.insert(
          (i.0).to_string(),
          SettingsEntry {
            name: (i.1).to_string(),
            value: (i.2).to_string(),
            visible: (i.3),
          },
        );
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
        self.settings.insert(
          v.get(0)?,
          SettingsEntry {
            name: v.get(1)?,
            value: v.get(2)?,
            visible: v.get(3)?,
          },
        );
      }
      println!("Loaded settings into memory\n");
    }

    Ok(())
  }

  pub fn get(&self) -> Result<HashMap<String, SettingsEntry>, Error> {
    Ok(self.settings.clone())
  }

  pub fn write(&mut self, id: &str, val: &str, conn: &Connection) -> Result<(), Error> {
    // Update DB
    let mut stmt = conn.prepare(
      "update settings set
          value=?1,
        where id=?2",
    )?;
    stmt.execute((id, val))?;

    // Update local settings map
    self.settings.get_mut(id).unwrap().value = val.to_string();

    Ok(())
  }
}
