use std::{str::FromStr, io::{Error}, io::ErrorKind};

// ch02/src/main.rs
#[derive(Debug)]
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

//  impl std::fmt::Display for Question {
//      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//          write!(f, "{}: {}", self.title, self.content);
//      }
//  }

#[derive(Debug)] 
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
 
 fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"), 
        "First Question".to_string(), 
        "Content of question".to_string(), 
        Some(vec!["faq".to_string()])
    );
    println!("{:?}", question);
 }