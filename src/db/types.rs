use rusqlite::Connection;
use std::collections::HashMap;
use std::vec;

#[derive(Debug, Clone)]
pub struct UserSettings {
  pub first_open: u8,
  pub name: String,
  pub target_cals: u16,
  pub target_carbs: u16,
}

#[derive(Debug)]
pub struct NutritionData {
  pub name: String,
  pub calories: u16,
  pub carbs: u16,
  pub sodium: u16,
}

#[derive(Debug)]
pub struct Entry {
  pub id: u16,
  pub name: String,
  pub time: String,
  pub nutrition: NutritionData,
}

#[derive(Debug)]
pub struct DailyInfo {
  pub date: String,
  pub entries: Vec<Entry>,
}

pub struct SettingsManager {
  pub settings: HashMap<String, [String; 2]>,
}

pub struct DatabaseManager {
  pub settings: SettingsManager,
  pub nutri_data: Vec<NutritionData>,
  pub history: Vec<DailyInfo>,

  pub db_path: String,
  pub conn: Connection,
}
