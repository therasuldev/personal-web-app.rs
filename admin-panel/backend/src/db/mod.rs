use rusqlite::Connection;

pub fn create_db_connection() -> Connection {
    dotenv::dotenv().ok();
    
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    
    Connection::open(db_url).expect("Failed to connect to database")
}
