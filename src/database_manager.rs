use std::vec;
use rusqlite::Connection;
use anyhow::Error;

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

pub struct DatabaseManager {
    settings: UserSettings,
    nutri_data: Vec<NutritionData>,
    history: Vec<DailyInfo>,

    db_path: String,
    conn: Connection,
}

impl DatabaseManager {
    pub fn new(db_path: String) -> Self {
        
        let c = Connection::open(&db_path).unwrap();

        Self {
            settings: UserSettings {
                first_open: 1,
                name: String::new(),
                target_cals: 2000,
                target_carbs: 20,
            },
            nutri_data: Vec::new(),
            history: Vec::new(),
            db_path: db_path,
            conn: c,
        }
    }

    pub fn load_settings(&mut self) -> Result<(), Error> {
        self.conn.execute(
            "create table if not exists settings (
                firstOpen bit,
                name varchar(255),
                targetCals int,
                targetCarbs int
            )",
            (),
        )?;

        let stmt = self.conn.prepare(
            "select * from settings"
        );
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

    pub fn get_settings(&self) -> Result<UserSettings, Error> {
      Ok(self.settings.clone())
    }

    pub fn write_settings(&mut self, settings: &UserSettings) -> Result<(), Error> {
        self.conn.execute(
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
              settings.target_carbs
            ),
        )?;

        self.settings = settings.clone();

        Ok(())
    }
}
