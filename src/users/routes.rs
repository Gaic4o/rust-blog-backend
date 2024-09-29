use warp::Filter;
use super::handlers;
use super::models::User;

pub fn user_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let users_base = warp::path("users");

    let create_user = users_base
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::create_user);

    let get_user = users_base
        .and(warp::path::param::<i32>())
        .and(warp::get())
        .and_then(handlers::get_user);

    create_user.or(get_user)
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
