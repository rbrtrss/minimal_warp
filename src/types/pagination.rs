use handle_errors::Error;
use crate::store::Store;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

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
