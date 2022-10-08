use warp::{body::BodyDeserializeError, cors::CorsForbidden, hyper::StatusCode, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    InvalidFlightPath,
}
impl warp::reject::Reject for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidFlightPath => write!(f, "Invalid Flight Path"),
        }
    }
}
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Path not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
