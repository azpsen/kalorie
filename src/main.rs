use anyhow::Error;
use std::collections::HashMap;
use std::{io, thread, time::Duration};

use crate::db::types::{DatabaseManager, FoodEntry, NutritionEntry};

pub mod db;
mod ui;

fn populate_entries(db_man: &mut DatabaseManager) -> Result<(), Error> {
  let tests = [
    "2012-12-12T12:12:12Z",
    "2018-12-12T12:12:12Z",
    "2023-01-12T12:12:12Z",
    "2023-01-13T12:12:12Z",
    "2023-01-01T12:12:12Z",
  ];
  for t in tests.iter() {
    let entry = FoodEntry {
      name: "testing".to_string(),
      datetime: t.parse::<chrono::DateTime<chrono::Utc>>()?,
      amount: 100.0,
      nutrition_id: 1,
      nutrition_data: None,
    };
    db_man.insert_into_journal(&entry)?;
  }

  Ok(())
}

fn populate_nutrition(db_man: &mut DatabaseManager) -> Result<(), Error> {
  let avocado = NutritionEntry {
    name: "Avocado".to_string(),
    amount: 100.0,
    serv_size: 200.0,
    data: HashMap::from([
      ("calories".to_string(), 160.0),
      ("protein".to_string(), 2.0),
      ("fat_total".to_string(), 14.7),
      ("carbs_total".to_string(), 8.5),
      ("fiber".to_string(), 6.7),
      ("carbs_net".to_string(), 1.8),
      ("sodium".to_string(), 7.0),
      ("potassium".to_string(), 485.0),
    ]),
  };

  println!("Populating nutrition database...");
  db_man.insert_into_nutridata(&avocado)?;

  Ok(())
}

fn test_entries(db_man: &mut DatabaseManager) -> Result<(), Error> {
  let journal = db_man.get_journal_for_day(chrono::Utc::now().date_naive())?;
  println!("\nJournal for today:\n");
  for (k, v) in journal {
    println!("{k}: {:#?}", v);
  }

  let journal = db_man.get_journal_for_week(2023, 2)?;
  println!("Journal for week 2 of 2023:\n");
  for (k, v) in journal {
    println!("{k}: {:#?}", v);
  }

  let journal = db_man.get_journal_for_month(2023, 1)?;
  println!("\nJournal for month 1 of 2023:\n");
  for (k, v) in journal {
    println!("{k}: {:#?}", v);
  }

  let journal = db_man.get_journal_for_year(2018)?;
  println!("\nJournal for all of 2018:\n");
  for (k, v) in journal {
    println!("{k}: {:#?}", v);
  }

  let journal = db_man.get_journal_for_range(
    chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
    chrono::NaiveDate::from_ymd_opt(2023, 5, 12).unwrap(),
  )?;
  println!("\nJournal for 2000-today:\n");
  for (k, v) in journal {
    println!("{k}: {:#?}", v);
  }

  Ok(())
}

fn test_nutrition(db_man: &mut DatabaseManager) -> Result<(), Error> {
  println!("Testing nutrition database...");
  let avocado = db_man.get_nutridata(1)?;
  println!("{:#?}", avocado);

  println!("Testing nutrition value conversion...");
  let avocado_500g = avocado.vals_per_amt(500.0);
  println!("Avocado macros per 500g: {:#?}", avocado_500g);

  println!("Testing nutrition value per serving...");
  let avocado_serv = avocado.per_serving();
  println!("Avocado macros per serving: {:#?}", avocado_serv);

  Ok(())
}

fn main() -> Result<(), Error> {
  // Load database
  let mut db_man = DatabaseManager::new("kalorie.db".to_string())?;

  let settings = db_man.settings.get()?;
  println!("Loaded settings:\n{:#?}", settings);

  populate_entries(&mut db_man)?;
  populate_nutrition(&mut db_man)?;

  test_entries(&mut db_man)?;
  test_nutrition(&mut db_man)?;

  Ok(())
}
