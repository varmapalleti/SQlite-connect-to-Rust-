use std::fs;
use std::path::Path;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;

    let qry = "
    PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS students
    (
        st_id               INTEGER PRIMARY KEY NOT NULL,
        names               TEXT                NOT NULL,
        created             DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated             DATETIME DEFAULT    (datetime('now', 'localtime')),
        done                BOOLEAN             NOT NULL DEFAULT 0  
    );
    CREATE TABLE IF NOT EXISTS projects
    (
        project_id          INTEGER PRIMARY KEY AUTOINCREMENT,
        product_name        TEXT,
        created             DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated             DATETIME DEFAULT    (datetime('now', 'localtime')),
        img_dir             TEXT NOT NULL,
        out_dir             TEXT NOT NULL,
        status              TEXT NOT NULL,
        st_id               INTEGER NOT NULL DEFAULT 1,
        FOREIGN KEY (st_id) REFERENCES students(st_id) ON DELETE SET NULL
    );
    ";

    let result = sqlx::query(qry).execute(&pool).await;
    pool.close().await;
    result
}

#[async_std::main]
async fn main() {
    let db_path = "dbconnect.db";
    let db_url = format!("sqlite://{}", db_path);

    // Check if database file exists
    if !Path::new(db_path).exists() {
        println!("Database file not found, creating a new one...");
        fs::File::create(db_path).expect("Failed to create database file");
    }

    // Create schema if needed
    match create_schema(&db_url).await {
        Ok(_) => println!("Database schema created successfully"),
        Err(e) => panic!("Failed to create schema: {}", e),
    }

    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO students(names) VALUES(?)";
    let result = sqlx::query(qry).bind("varma").execute(&instances).await;

    instances.close().await;
    println!("{:?}", result);
}
