use tokio_postgres::{Client, NoTls, Error};
use std::env;

pub async fn connect_db() -> Result<Client, Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("데이터베이스 연결 오류: {}", e); 
        }
    });
    Ok(client) 
}

pub async fn init_db() -> Result<(), Error> {
    let client = connect_db().await?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            password VARCHAR NOT NULL
        )
        ",
    )
    .await?;
    Ok(())
}
