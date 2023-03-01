use serde::{Serialize, Deserialize};
use crate::types::question;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: question::QuestionId,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct AnswerId(pub String);