use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UserSettings {
  pub first_open: u8,
  pub name: String,
  pub target_cals: u16,
  pub target_carbs: u16,
}

#[derive(Debug)]
pub struct NutritionData {
  pub id: u16,
  pub name: String,
  pub calories: u16,
  pub carbs: u16,
  pub sodium: u16,
}

#[derive(Debug, Clone)]
pub struct FoodEntry {
  pub name: String,
  pub datetime: DateTime<Utc>,
  pub nutrition: u16,
}

#[derive(Debug)]
pub struct DailyInfo {
  pub date: String,
  pub entries: Vec<FoodEntry>,
}

#[derive(Debug, Clone)]
pub struct SettingsEntry {
  pub name: String,
  pub value: String,
  pub visible: bool,
}

pub struct SettingsManager {
  pub settings: HashMap<String, SettingsEntry>,
}

pub struct JournalManager {
  pub journal: HashMap<u16, FoodEntry>,
}

pub struct DatabaseManager {
  pub settings: SettingsManager,
  pub nutri_data: Vec<NutritionData>,
  pub history: Vec<DailyInfo>,

  pub db_path: String,
  pub conn: Connection,
}
