use warp::{
    body::BodyDeserializeError, cors::CorsForbidden, http::StatusCode, reject::Reject, Rejection,
    Reply,
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    OutOfBounds,
    WrongRange,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameters {}", err)
            }
            Error::MissingParameters => {
                write!(f, "Missing parameter")
            }
            Error::OutOfBounds => {
                write!(f, "Requested index are out of bounds")
            }
            Error::WrongRange => {
                write!(f, "Wrong range")
            }
            Error::QuestionNotFound => {
                write!(f, "Question does not exist")
            }
        }
    }
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::BAD_REQUEST,
        ))
    // } else if let Some(error) = r.find::<Error::MissingParameters>() {
    //     Ok(warp::reply::with_status(
    //         error.to_string(),
    //         StatusCode::BAD_REQUEST,
    //     ))
    // } else if let Some(error) = r.find::<Error::OutOfBounds>() {
    //     Ok(warp::reply::with_status(
    //         error.to_string(),
    //         StatusCode::BAD_REQUEST,
    //     ))    
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
