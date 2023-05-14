use anyhow::Error;
use rusqlite::Connection;

use super::types::*;

impl NutritionManager {
  pub fn new() -> Self {
    Self {}
  }

  pub fn load(&mut self, conn: &Connection) -> Result<(), Error> {
    // Check if 'nutrition' table exists
    let mut nutrition_table =
      conn.prepare("select * from sqlite_master where type='table' and name='nutrition'")?;

    if !nutrition_table.exists(())? {
      // Create nutrition table in db
      println!("Nutrition table does not exist, creating...");
      conn.execute(
        "create table nutrition (
          id integer primary key autoincrement,
          name text,
          serv_size real,
          calories integer,
          protein real,
          fat_total real,
          fat_sat real,
          fat_trans real,
          cholesterol real,
          carbs_total real,
          fiber real,
          sugar real,
          carbs_net real,
          sodium real,
          potassium real
        )",
        (),
      )?;

      println!("Created table nutrition");
    }

    Ok(())
  }

  pub fn get(&self, conn: &Connection, id: u16) -> Result<NutritionEntry, Error> {
    let stmt = conn.prepare("select * from nutrition where id=?1");
    let mut statement = stmt?;

    let mut data = NutritionEntry::new();
    let mut nutrition_vals = statement.query(&[&id])?;

    while let Some(v) = nutrition_vals.next()? {
      // TODO mapping option?
      data.name = v.get(1)?;
      data.serv_size = v.get(2)?;
      data.calories = v.get(3)?;
      data.protein = v.get(4)?;
      data.fat_total = v.get(5)?;
      data.fat_sat = v.get(6)?;
      data.fat_trans = v.get(7)?;
      data.cholesterol = v.get(8)?;
      data.carbs_total = v.get(9)?;
      data.fiber = v.get(10)?;
      data.sugar = v.get(11)?;
      data.carbs_net = v.get(12)?;
      data.sodium = v.get(13)?;
      data.potassium = v.get(14)?;
    }

    Ok(data)
  }

  pub fn insert(&self, conn: &Connection, entry: &NutritionEntry) -> Result<(), Error> {
    // TOOD mapping option?
    println!("Inserting...");
    let mut stmt = conn.prepare(
      "insert into nutrition (
        name,
        serv_size,
        calories,
        protein,
        fat_total,
        fat_sat,
        fat_trans,
        cholesterol,
        carbs_total,
        fiber,
        sugar,
        carbs_net,
        sodium,
        potassium
      ) values (
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14
      )",
    )?;

    println!("Statement prepared, executing...");
    stmt.execute((
      &entry.name,
      &entry.serv_size,
      &entry.calories,
      &entry.protein,
      &entry.fat_total,
      &entry.fat_sat,
      &entry.fat_trans,
      &entry.cholesterol,
      &entry.carbs_total,
      &entry.fiber,
      &entry.sugar,
      &entry.carbs_net,
      &entry.sodium,
      &entry.potassium,
    ))?;

    println!("Statement executed successfully!");

    Ok(())
  }
}
