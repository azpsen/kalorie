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
pub struct NutritionEntry {
  pub name: String,
  pub serv_size: f64,
  pub calories: u16,
  pub protein: Option<f64>,
  pub fat_total: Option<f64>,
  pub fat_sat: Option<f64>,
  pub fat_trans: Option<f64>,
  pub cholesterol: Option<f64>,
  pub carbs_total: Option<f64>,
  pub fiber: Option<f64>,
  pub sugar: Option<f64>,
  pub carbs_net: Option<f64>,
  pub sodium: Option<f64>,
  pub potassium: Option<f64>,
}

impl NutritionEntry {
  pub fn new() -> Self {
    Self {
      name: "".to_string(),
      serv_size: 100.0,
      calories: 0,
      protein: None,
      fat_total: None,
      fat_sat: None,
      fat_trans: None,
      cholesterol: None,
      carbs_total: None,
      fiber: None,
      sugar: None,
      carbs_net: None,
      sodium: None,
      potassium: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FoodEntry {
  pub name: String,
  pub datetime: DateTime<Utc>,
  pub nutrition_id: u16,
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

pub struct JournalManager {}

pub struct NutritionManager {}

pub struct DatabaseManager {
  pub settings: SettingsManager,
  pub nutri_data: NutritionManager,
  pub journal: JournalManager,

  pub db_path: String,
  pub conn: Connection,
}
