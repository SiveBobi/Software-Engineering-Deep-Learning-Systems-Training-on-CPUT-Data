use crate::model::TransformerQA;

pub fn answer_question<B>(
    model: &TransformerQA<B>,
    question: &str,
) -> String {
    // Dummy inference for prototype
    if question.to_lowercase().contains("graduation") {
        "31 December 2026".to_string()
    } else if question.to_lowercase().contains("hdc") {
        "5 meetings".to_string()
    } else {
        "Answer not found in dataset".to_string()
    }
}
