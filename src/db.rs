use bcrypt::{hash, DEFAULT_COST, verify};
use sqlx::{Connection, SqliteConnection, Executor, SqlitePool};

use crate::models::{User, CreateUser, AuthUser};


// pub type UserKey = String;
// pub type EntryDatabaseModel = HashMap<UserKey, Vec<KilometerEntry>>;
// pub type UserDatabaseModel = HashMap<UserKey, User>;

#[derive(Debug, Clone)]
pub struct DatabaseModel {
    // pub entries: EntryDatabaseModel,
    // pub users: UserDatabaseModel,
    // pub database_version: DatabaseVersion,
}

#[derive(Clone)]
pub struct Database {
    pub conn: SqlitePool,
  //pub database: Arc<RwLock<DatabaseModel>>,
}

impl Database {
  pub async fn auth_user(self, user: AuthUser) -> bool {
    let hash: (String, ) = sqlx::query_as("SELECT hash FROM users WHERE username = ?").bind(user.username).fetch_one(&self.conn).await.unwrap_or(("".to_owned(),));
    if hash.0.is_empty() {
      return false;
    }
    
    verify(user.password, &hash.0).unwrap()
  }

  pub async fn create_user(self, user: CreateUser) {
    let hash = hash(user.password, DEFAULT_COST).unwrap();
    sqlx::query("INSERT INTO users
    (username, email, hash)
    VALUES (?, ?, ?)").bind(user.username).bind(user.email).bind(hash)
    .execute(&self.conn).await.unwrap();
  }

  pub async fn get_meters_sum(self, username: &str) -> i32 {
    let meters_vec = sqlx::query_as::<_, MeterData>("SELECT meters FROM data WHERE username = ?").bind(username).fetch_all(&self.conn).await.unwrap_or(Vec::new());
    // if meters_vec.is_empty(){ return 0;}

    let mut sum = 0;
    for d in meters_vec {
      sum += d.meters;
    }

    return sum;
  }
}

#[derive(sqlx::FromRow)]
struct MeterData {
  id: i32, user_id: i32, time: String, meters: i32,
}

pub async fn init_db() -> Database {
  println!("initializing database");
  let con = SqlitePool::connect("sqlite:db/db.db").await.unwrap();
  sqlx::query("CREATE TABLE IF NOT EXISTS users (
    id                INTEGER PRIMARY KEY,
    username          TEXT NOT NULL,
    email             TEXT NOT NULL,
    hash              TEXT NOT NULL
    )").execute(&con).await.unwrap();

  sqlx::query("CREATE TABLE IF NOT EXISTS data (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    meters UNSIGNED BIG INT NOT NULL,
    FOREIGN KEY(username) REFERENCES users(username)
    )").execute(&con).await.unwrap();

    let username = "testname";
    let meters = 10;
    
    sqlx::query("INSERT INTO data
    (username, meters)
    VALUES (?, ?)").bind(username).bind(meters).execute(&con).await.unwrap();
  
  Database { conn: con }
}