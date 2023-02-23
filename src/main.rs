// ch02/src/main.rs

use core::fmt;

struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
 }
 
 struct QuestionId(String);
 
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

 impl fmt::Display for Question {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "{}: {}", self.title, self.content);
     }
 }
 
 fn main() {
    let question = Question::new(
        QuestionId("1".to_string()), 
        "First Question".to_string(), 
        "Content of question".to_string(), 
        Some(vec!["faq".to_string()])
    );
    println!("{}", question);
 }