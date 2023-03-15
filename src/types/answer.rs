use crate::types::question;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: question::QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: question::QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct AnswerId(pub i32);
