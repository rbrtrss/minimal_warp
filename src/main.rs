use serde::{Serialize, Deserialize};
use std::{
    // io::{Error, ErrorKind},
    // str::FromStr, 
    collections::HashMap,
};
use warp::{http::Method, http::StatusCode, 
    // reject::Reject, 
    Filter, 
    Rejection, 
    Reply, 
    cors::CorsForbidden, reject::Reject};

// ch02/src/main.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

// impl Question {
//     fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
//         Question {
//             id,
//             title,
//             content,
//             tags,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
struct QuestionId(String);

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    OutOfBounds,
    WrongRange,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameters {}", err)
            },
            Error::MissingParameters => {
                write!(f, "Missing parameter")
            },
            Error::OutOfBounds => {
                write!(f, "Requested index are out of bounds")
            },
            Error::WrongRange => {
                write!(f, "Wrong range")
            },
        }
    }
}

impl Reject for Error {}


#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String,String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
    return Ok(Pagination { 
        start: params.get("start").unwrap().parse::<usize>().map_err(Error::ParseError)?,
        end: params.get("end").unwrap().parse::<usize>().map_err(Error::ParseError)?,
    });
    }
    Err(Error::MissingParameters)
}
// impl FromStr for QuestionId {
//     type Err = std::io::Error;

//     fn from_str(id: &str) -> Result<Self, Self::Err> {
//         match id.is_empty() {
//             false => Ok(QuestionId(id.to_string())),
//             true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
//         }
//     }
// }

// #[derive(Debug)]
// struct InvalidId;
// impl Reject for InvalidId {}

fn extract_questions(pagination: Pagination, store: Store) -> Result<&[Question], Error> {
        if pagination.start > store.questions.len() || pagination.end > store.questions.len() {
            // println!("{}, {}", pagination.end, params.len());
            return Err(Error::OutOfBounds);
        } else if pagination.start > pagination.end {
            return Err(Error::WrongRange);
        } else {
            let res: Vec<Question> = store.questions.values().cloned().collect();
            let res = &res[pagination.start..pagination.end];
            return Ok(res)
        }
}

async fn get_questions(params: HashMap<String,String>,store: Store) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
    // let question = Question::new(
    //     QuestionId::from_str("1").expect("No id provided"),
    //     "First Question".to_string(),
    //     "Content of question".to_string(),
    //     Some(vec!["faq".to_string()]),
    // );

    // match question.id.0.parse::<i32>() {
    //     Err(_) => Err(warp::reject::custom(InvalidId)),
    //     Ok(_) => Ok(warp::reply::json(&question)),
    // }
    // let mut start = 0;
    // match params.get("start") {
    //     Some(start) => println!("{}", start),
    //     None => println!("No starting value")
    // }
    // if let Some(n) = params.get("start") {
    //     start = n.parse::<usize>().expect("cannot parse start")
    // }

    // println!("{}", start);
    // let res: Vec<Question> = store.questions.values().cloned().collect();
    // Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    // println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(error.to_string(), StatusCode::RANGE_NOT_SATISFIABLE))
    } else if let Some(error) = r.find::<CorsForbidden>()  {
        Ok(warp::reply::with_status(
            error.to_string(), 
            StatusCode::FORBIDDEN))
        
    // } else if let Some(InvalidId) = r.find() {
    //     Ok(warp::reply::with_status(
    //         "No valid ID presented".to_string(),
    //         StatusCode::UNPROCESSABLE_ENTITY,
    //     ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store { questions: Self::init() }
    }

    fn init() -> HashMap<QuestionId,Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("cannot read questions.json")
    }

    // fn add_question(mut self, question: Question) -> Self {
    //     self.questions.insert(question.id.clone(), question);
    //     self
    // }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::POST, Method::GET, Method::DELETE]);

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(get_questions);

    

    // let hello = warp::path("hello")
    //     .map(|| format!("Hola mundo!!!"));
    let routes = get_questions.with(cors).recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
