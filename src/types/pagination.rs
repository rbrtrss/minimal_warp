use crate::store::Store;
use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from quey params
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first question to be returned
    pub start: usize,
    /// The index of the last question to be returned
    pub end: usize,
}

/// Extract query parameters from the `/questions` route
/// Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=0&end=1`
pub async fn extract_pagination(
    params: HashMap<String, String>,
    store: Store,
) -> Result<Pagination, Error> {
    let store_size = store.questions.read().await.len();
    if params.contains_key("start") && params.contains_key("end") {
        let pagination = Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        };
        // Check against store size for correctness of start and end parameters
        if pagination.start > store_size || pagination.end > store_size {
            // println!("{}, {}", pagination.end, params.len());
            return Err(Error::OutOfBounds);
        } else if pagination.start > pagination.end {
            return Err(Error::WrongRange);
        } else {
            return Ok(pagination);
        }
    }
    Err(Error::MissingParameters)
}
