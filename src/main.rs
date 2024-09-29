use warp::Filter;

mod db;
mod users;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = db::init_db().await {
        eprintln!("Failed to initialize database: {}", e);
        return;
    }

    let user_routes = users::routes::user_routes();


    let routes = user_routes
        .with(warp::log("api"));

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
