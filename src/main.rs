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
    cors::CorsForbidden};

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

async fn get_questions(params: HashMap<String,String>,store: Store) -> Result<impl Reply, Rejection> {
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
    let mut start = 0;
    // match params.get("start") {
    //     Some(start) => println!("{}", start),
    //     None => println!("No starting value")
    // }
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("cannot parse start")
    }

    println!("{}", start);
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_errors(r: Rejection) -> Result<impl Reply, Rejection> {
    // println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>()  {
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
    let routes = get_questions.with(cors).recover(return_errors);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
