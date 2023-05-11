use anyhow::Error;
use rusqlite::Connection;
use std::collections::HashMap;

use super::types::*;

// Cumbersome declaration, TODO find solution not involving to_string()
const DEFAULT_SETTINGS: [[&'static str; 3]; 4] = [
  ["first_open", "First Open", "1"],
  ["name", "Name", ""],
  ["target_cals", "Target Calories", "2000"],
  ["target_carbs", "Target Carbohydrates", "20"],
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
          name text not null,
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
            id,
            name,
            value
          )
          values (
            ?1, ?2, ?3
          )",
          (i[0], i[1], i[2]),
        )?;

        // Load default settings into memory
        self
          .settings
          .insert(i[0].to_string(), [i[1].to_string(), i[2].to_string()]);
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
        self.settings.insert(v.get(0)?, [v.get(1)?, v.get(2)?]);
      }
      println!("Loaded settings into memory\n");
    }

    Ok(())
  }

  pub fn get(&self) -> Result<HashMap<String, String>, Error> {
    let mut settings_map: HashMap<String, String> = HashMap::new();
    for (k, v) in self.settings.iter() {
      settings_map.insert(k.to_string(), v[1].to_string());
    }
    Ok(settings_map)
  }

  pub fn get_name(&self, id: String) -> Result<String, Error> {
    match self.settings.get(&id) {
      Some(val) => return Ok(val[0].to_string()),
      None => return Ok("Not Found".to_string()), // TODO replace with proper error
    }
  }

  pub fn write(
    &mut self,
    settings: &HashMap<String, String>,
    conn: &Connection,
  ) -> Result<(), Error> {
    for (k, v) in settings {
      let mut stmt = conn.prepare("update settings set value = ?1 where id = ?2")?;
      stmt.execute((v, k))?;

      let pretty_name = match self.settings.get(&k.to_string()) {
        Some(val) => val[0].to_string(),
        None => "Not Found".to_string(), // TODO replace with proper error
      };
      self
        .settings
        .insert(k.to_string(), [pretty_name, v.to_string()]);
    }

    Ok(())
  }
}
