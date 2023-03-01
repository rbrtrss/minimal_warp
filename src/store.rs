    use tokio::sync::RwLock;
    use std::sync::Arc;
    use std::collections::HashMap;
    use crate::types::{
        question::{QuestionId, Question},
        answer::{AnswerId, Answer},
    };

    #[derive(Clone)]
    pub struct Store {
        pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
        pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    }
    
    impl Store {
        pub fn new() -> Self {
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
