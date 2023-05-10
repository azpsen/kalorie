use anyhow::Error;
use rusqlite::Connection;

use super::types::*;

impl SettingsManager {
  pub fn new() -> Self {
    Self {
      settings: UserSettings {
        first_open: 1,
        name: String::new(),
        target_cals: 2000,
        target_carbs: 20,
      },
    }
  }

  pub fn load(&mut self, conn: &Connection) -> Result<(), Error> {
    conn.execute(
      "create table if not exists settings (
        firstOpen bit,
        name varchar(255),
        targetCals int,
        targetCarbs int
      )",
      (),
    )?;

    let stmt = conn.prepare("select * from settings");
    let mut stmt_bind = stmt?;
    let contents = stmt_bind.query_map([], |row| {
      Ok(UserSettings {
        first_open: row.get(0)?,
        name: row.get(1)?,
        target_cals: row.get(2)?,
        target_carbs: row.get(3)?,
      })
    })?;

    for c in contents {
      self.settings = c.unwrap();
      return Ok(()); // Ugly solution to only get one row, TODO find elegant solution
    }

    Ok(())
  }

  pub fn get(&self) -> Result<UserSettings, Error> {
    Ok(self.settings.clone())
  }

  pub fn write(&mut self, settings: &UserSettings, conn: &Connection) -> Result<(), Error> {
    conn.execute(
      "insert into settings (
        firstOpen,
        name,
        targetCals,
        targetCarbs
      )
      values (
        ?1, ?2, ?3, ?4
      )",
      (
        settings.first_open,
        settings.name.clone(),
        settings.target_cals,
        settings.target_carbs,
      ),
    )?;

    self.settings = settings.clone();

    Ok(())
  }
}
