use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}




const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT Found\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";



fn main() {

    if let Err(e) = set_database() {
        eprintln!("데이터베이스 설정 오류: {}", e);
        return;
    }


    let listener = TcpListener::bind("0.0.0.0:8000").expect("포트 8000에 바인드 실패");
    println!("서버가 8000번 포트에서 시작되었습니다");


    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream);
            }
            Err(e) => {
                eprintln!("연결 오류: {}", e);
            }
        }
    }
}


fn set_database() -> Result<(), PostgresError> {

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 환경 변수가 설정되어 있어야 합니다");
    let mut client = Client::connect(&db_url, NoTls)?;


    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )",
        &[],
    )?;

    Ok(())
}


fn handle_client(stream: &mut TcpStream) {
  
    let mut buffer = [0; 512];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("스트림에서 읽기 실패: {}", e);
        return;
    }


    if let Err(e) = stream.write_all(OK_RESPONSE.as_bytes()) {
        eprintln!("스트림에 쓰기 실패: {}", e);
    }
}