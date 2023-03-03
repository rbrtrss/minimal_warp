use handle_errors::Error;
use crate::store::Store;
use crate::types::{
    pagination::extract_pagination,
    question::{Question, QuestionId},
};
use std::collections::HashMap;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn get_questions(
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

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Added question", StatusCode::OK))
}

pub async fn update_question(
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

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question Deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
