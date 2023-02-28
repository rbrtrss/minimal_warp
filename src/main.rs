use serde::{Deserialize, Serialize};
use std::{
    // io::{Error, ErrorKind},
    // str::FromStr,
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::RwLock;
use warp::{
    body::BodyDeserializeError,
    cors::CorsForbidden,
    http::Method,
    http::StatusCode,
    reject::Reject,
    // reject::Reject,
    Filter,
    Rejection,
    Reply,
};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
struct AnswerId(String);

#[derive(Debug)]
enum Error {
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

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

async fn extract_pagination(
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

// fn extract_questions(pagination: Pagination, store: Store) -> Result<&'static [Question], Error> {
//         if pagination.start > store.questions.len() || pagination.end > store.questions.len() {
//             // println!("{}, {}", pagination.end, params.len());
//             return Err(Error::OutOfBounds);
//         } else if pagination.start > pagination.end {
//             return Err(Error::WrongRange);
//         } else {
//             let res: Vec<Question> = store.questions.values().cloned().collect();
//             let res = &res[pagination.start..pagination.end];
//             return res
//         }
// }

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params, store.clone()).await?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
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
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
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

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("cannot read questions.json")
    }

    // fn add_question(mut self, question: Question) -> Self {
    //     self.questions.insert(question.id.clone(), question);
    //     self
    // }
}

async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Added question", StatusCode::OK))
}

async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId( params.get("questionId").unwrap().to_string()),
    };

    store.answers.write().await.insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}

async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question Updated", StatusCode::OK))
}

async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question Deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
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
        .and(store_filter.clone())
        .and_then(get_questions);

    let add_questions = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_questions = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_questions = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer);
    // let hello = warp::path("hello")
    //     .map(|| format!("Hola mundo!!!"));
    let routes = get_questions
        .or(add_questions)
        .or(update_questions)
        .or(delete_questions)
        .or(add_answer)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
