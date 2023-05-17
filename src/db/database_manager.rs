use anyhow::Error;
use chrono::NaiveDate;
use rusqlite::Connection;
use std::collections::HashMap;

use crate::db::{journal_manager::JournalManager, nutrition_manager::NutritionManager};

use super::types::*;

impl DatabaseManager {
  pub fn new(db_path: String) -> Result<Self, Error> {
    println!("Opening database {db_path}...");
    let c = Connection::open(&db_path)?;
    println!("Successfully opened database\n");

    let mut s = Self {
      settings: SettingsManager::new(),
      db_path: db_path,
      conn: c,
    };
    s.settings.load(&s.conn)?;

    JournalManager::load(&s.conn)?;
    NutritionManager::load(&s.conn)?;
    Ok(s)
  }

  /* SETTINGS MANAGEMENT */
  pub fn write_settings(&mut self, id: &str, val: &str) -> Result<(), Error> {
    self.settings.write(id, val, &self.conn)
  }

  pub fn get_settings(&self) -> Result<HashMap<String, SettingsEntry>, Error> {
    self.settings.get()
  }

  /* JOURNAL MANAGEMENT */
  pub fn insert_into_journal(&mut self, entry: &FoodEntry) -> Result<(), Error> {
    JournalManager::insert(entry, &self.conn)
  }

  pub fn get_journal_for_day(&mut self, day: NaiveDate) -> Result<HashMap<u16, FoodEntry>, Error> {
    JournalManager::get_day(&self.conn, day)
  }

  pub fn get_journal_for_week(
    &mut self,
    year: u16,
    week: u16,
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    JournalManager::get_week(&self.conn, year, week)
  }

  pub fn get_journal_for_month(
    &mut self,
    year: u16,
    month: u16,
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    JournalManager::get_month(&self.conn, year, month)
  }

  pub fn get_journal_for_year(&mut self, year: u16) -> Result<HashMap<u16, FoodEntry>, Error> {
    JournalManager::get_year(&self.conn, year)
  }

  pub fn get_journal_for_range(
    &mut self,
    begin_date: NaiveDate,
    end_date: NaiveDate,
  ) -> Result<HashMap<u16, FoodEntry>, Error> {
    JournalManager::get_range(&self.conn, begin_date, end_date)
  }

  /* NUTRITION MANAGEMENT */
  pub fn get_nutridata(&mut self, id: u16) -> Result<NutritionEntry, Error> {
    NutritionManager::get(&self.conn, id)
  }

  pub fn insert_into_nutridata(&mut self, entry: &NutritionEntry) -> Result<(), Error> {
    NutritionManager::insert(&self.conn, entry)
  }
}
