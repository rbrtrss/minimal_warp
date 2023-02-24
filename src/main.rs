use std::{str::FromStr, io::{Error, ErrorKind}};
use warp::{Filter, Rejection, Reply, http::StatusCode, reject::Reject};
use serde::Serialize;

// ch02/src/main.rs
#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
 }

 impl Question {
    fn new(
        id: QuestionId, 
        title: String, 
        content: String, 
        tags: Option<Vec<String>>
     ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
 }

#[derive(Debug, Serialize)] 
 struct QuestionId(String);
 
impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id provided")
            ),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl Reply, Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"), 
        "First Question".to_string(), 
        "Content of question".to_string(), 
        Some(vec!["faq".to_string()])
    );

    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(
                &question
            ))
        }
    }
}

async fn return_errors(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND
        ))
    }
}
 
 #[tokio::main]
 async fn main() {
    
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_errors);

    // let hello = warp::path("hello")
    //     .map(|| format!("Hola mundo!!!"));

    let routes= get_items;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
 }