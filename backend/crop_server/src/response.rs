use warp::reply;

pub type Response = reply::WithStatus<reply::Json>;

pub fn new<T>(status: http::StatusCode, json: &T) -> Response
where
    T: serde::Serialize,
{
    reply::with_status(reply::json(json), status)
}
