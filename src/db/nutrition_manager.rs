pub mod NutritionManager {

  use anyhow::Error;
  use rusqlite::Connection;

  use crate::db::types::*;

  pub fn load(conn: &Connection) -> Result<(), Error> {
    // Check if 'nutrition' table exists
    let mut nutrition_table =
      conn.prepare("select * from sqlite_master where type='table' and name='nutrition'")?;

    if !nutrition_table.exists(())? {
      // Create nutrition table in db
      println!("Nutrition table does not exist, creating...");
      conn.execute(
        "create table nutrition (
          id integer primary key,
          name text,
          amount real,
          serv_size real,
          calories real,
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

  pub fn get(conn: &Connection, id: u16) -> Result<NutritionEntry, Error> {
    let stmt = conn.prepare("select * from nutrition where id=?1");
    let mut statement = stmt?;

    let mut result = NutritionEntry::new();
    let mut nutrition_vals = statement.query(&[&id])?;

    while let Some(v) = nutrition_vals.next()? {
      for (i, name) in NUTRITION_VALUES.iter().enumerate() {
        match v.get(i + 4)? {
          Some(x) => {
            result.data.insert(name.to_string(), x);
          }
          None => (),
        }
        result.name = v.get(1)?;
        result.amount = v.get(2)?;
        result.serv_size = v.get(3)?;
      }
    }

    Ok(result)
  }

  pub fn insert(conn: &Connection, entry: &NutritionEntry) -> Result<u16, Error> {
    // Insert nutrition entry into db, returns id of new entry
    let mut stmt = conn.prepare(
      "insert into nutrition (
        name,
        amount,
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
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15
      ) returning id",
    )?;

    println!("Statement prepared, executing...");
    let mut params = Vec::<Option<rusqlite::types::Value>>::new();
    params.push(Some(entry.name.clone().into()));
    params.push(Some(entry.amount.into()));
    params.push(Some(entry.serv_size.into()));
    for val in NUTRITION_VALUES {
      params.push(match entry.data.get(&val.to_string()) {
        Some(x) => Some((*x).into()),
        None => None,
      });
    }

    let mut result = stmt.query(rusqlite::params_from_iter(params))?;
    let mut id = 0;
    while let Some(row) = result.next()? {
      id = row.get(0)?;
    }

    println!("Statement executed successfully!");

    Ok(id)
  }
}
