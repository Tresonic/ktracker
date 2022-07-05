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
  
  Database { conn: con }
}