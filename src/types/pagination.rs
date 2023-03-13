use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from quey params
#[derive(Default, Debug)]
pub struct Pagination {
    /// The index of the first question to be returned
    pub limit: Option<u32>,
    /// The index of the last question to be returned
    pub offset: u32,
}

/// Extract query parameters from the `/questions` route
/// Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=0&end=1`
/// # Example usage
/// ```rust
/// use std::collections::HashMap;
///
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(1));
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
