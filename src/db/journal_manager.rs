pub mod JournalManager {

  use anyhow::Error;
  use chrono::NaiveDate;
  use rusqlite::Connection;
  use std::collections::HashMap;

  use crate::db::nutrition_manager::*;
  use crate::db::types::*;

  pub fn load(conn: &Connection) -> Result<(), Error> {
    // Check if 'journal' table exists
    let mut journal_table =
      conn.prepare("select * from sqlite_master where type='table' and name='journal'")?;

    if !journal_table.exists(())? {
      // Create journal table in db
      println!("Journal table does not exist, creating...");
      conn.execute(
        "create table journal (
          id integer primary key autoincrement,
          name text,
          entry_datetime text,
          amount real,
          nutrition_id integer
        )",
        (),
      )?;

      println!("Created table journal");
    }

    Ok(())
  }

  fn get_from_db<T: rusqlite::ToSql>(
    conn: &Connection,
    sql: &str,
    params: &[T],
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    let mut j = HashMap::new();
    let mut stmt = conn.prepare(&sql)?;
    let mut journal_vals = stmt.query(rusqlite::params_from_iter(params))?;

    while let Some(v) = journal_vals.next()? {
      j.insert(
        v.get(0)?,
        FoodEntry {
          name: v.get(1)?,
          datetime: v.get(2)?,
          amount: v.get(3)?,
          nutrition_id: v.get(4)?,
          nutrition_data: Some(NutritionManager::get(conn, v.get(4)?)?),
        },
      );
    }

    Ok(j)
  }

  pub fn get_day(conn: &Connection, day: NaiveDate) -> Result<HashMap<u16, FoodEntry>, Error> {
    println!("Loading for day {:#?}", day);
    Ok(get_from_db::<NaiveDate>(
      conn,
      "select * from journal where date(entry_datetime)=date(?1)",
      &[day],
    )?)
  }

  pub fn get_week(
    conn: &Connection,
    year: u16,
    week: u16,
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    Ok(get_from_db::<u16>(
      conn,
      "select * from journal where
        cast(strftime('%Y', entry_datetime) as str)=cast(?1 as str) and
        cast(strftime('%W', entry_datetime) as str)=cast(?2 as str)",
      &[year, week],
    )?)
  }

  pub fn get_month(
    conn: &Connection,
    year: u16,
    month: u16,
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    Ok(get_from_db::<u16>(
      conn,
      "select * from journal where
        cast(strftime('%Y', entry_datetime) as str)=cast(?1 as str) and
        cast(strftime('%m', entry_datetime) as str)=cast(?2 as str)",
      &[year, month],
    )?)
  }

  pub fn get_year(conn: &Connection, year: u16) -> Result<HashMap<u16, FoodEntry>, Error> {
    Ok(get_from_db::<u16>(
      conn,
      "select * from journal where
        cast(strftime('%Y', entry_datetime) as str)=cast(?1 as str)",
      &[year],
    )?)
  }

  pub fn get_range(
    conn: &Connection,
    begin_date: NaiveDate,
    end_date: NaiveDate,
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    Ok(get_from_db::<NaiveDate>(
      conn,
      "select * from journal where
        date(entry_datetime) between ?1 and ?2",
      &[begin_date, end_date],
    )?)
  }

  pub fn insert(entry: &FoodEntry, conn: &Connection) -> Result<(), Error> {
    let mut stmt = conn.prepare(
      "insert into journal ( 
        name,
        entry_datetime,
        amount,
        nutrition_id
      ) values (
        ?1, ?2, ?3, ?4
      )",
    )?;

    stmt.execute((
      &entry.name,
      &entry.datetime,
      &entry.amount,
      &entry.nutrition_id,
    ))?;

    Ok(())
  }
}
