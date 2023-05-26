struct Survey {
    question1: Option<i32>,    // number output question
    question2: Option<bool>,   // boolean question
    question3: Option<String>, // multiple choice question
}

fn main() {
    let response = Survey {
        // question1: Some(12),
        question1: None,

        // question2: Some(true),
        question2: Some(false),

        // question3: Some("A".to_owned()),
        question3: None,
    };

    match response.question1 {
        Some(answer) => println!("question 1 : {:?}", answer),
        None => println!("question 1: No answer"),
    }
    match response.question2 {
        Some(answer) => println!("question 2 : {:?}", answer),
        None => println!("question 2: No answer"),
    }
    match response.question3 {
        Some(answer) => println!("question 3 : {:?}", answer),
        None => println!("question 3: No answer"),
    }
}
