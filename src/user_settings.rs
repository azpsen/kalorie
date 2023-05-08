use rusqlite::{Connection, Error};

#[derive(Debug)]
pub struct UserSettings {
    pub firstOpen: u8,
    pub name: String,
    pub targetCals: u16,
    pub targetCarbs: u16,
}

const SETTINGS_PATH: &str = "userSettings.db";

pub fn load_settings() -> Result<UserSettings, Error> {
    println!("Attempting to load {}", SETTINGS_PATH);

    let conn = Connection::open(SETTINGS_PATH)?;

    conn.execute(
        "create table if not exists settings (
            firstOpen bit,
            name varchar(255),
            targetCals int,
            targetCarbs int,
        )",
        (),
    )?;

    let mut stmt = conn.prepare(
        "select * from settings"
    );
    let mut stmt_bind = stmt?;
    let contents = stmt_bind.query_map([], |row| {
        Ok(UserSettings {
            firstOpen: row.get(0)?,
            name: row.get(1)?,
            targetCals: row.get(2)?,
            targetCarbs: row.get(3)?,
        })
    })?;

    let mut rval = UserSettings { firstOpen: 1, name: "".to_string(), targetCals: 2000, targetCarbs: 20 };
    for c in contents {
        println!("Loaded {SETTINGS_PATH}, contents: {:?}", c.unwrap());
//        rval = c.unwrap();
    }

   Ok(rval)
}

pub fn write_settings(settings: UserSettings) -> Result<(), Error> {
    let conn = Connection::open(SETTINGS_PATH)?;

    conn.execute(
        "insert into settings (firstOpen, name, targetCals, targetCarbs) values (?1, ?2, ?3, ?4)",
        (settings.firstOpen, settings.name, settings.targetCals, settings.targetCarbs),
    )?;

    Ok(())
}
